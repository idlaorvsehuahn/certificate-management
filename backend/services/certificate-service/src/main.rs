use certificate_service::{app, config::AppConfig};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AppConfig::from_environment()?;
    shared::telemetry::init(&config.telemetry);

    let db_pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .connect(&config.database.url)
        .await?;
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    // Connect to NATS
    let nats_client = async_nats::connect(&config.nats.url).await?;
    info!("Connected to NATS on {}", config.nats.url);

    // Install default crypto provider for rustls
    rustls::crypto::ring::default_provider()
        .install_default()
        .ok();

    let address = config.server.address();
    let router = app::build_router(config.clone(), db_pool, nats_client);

    if config.tls.enabled {
        let certs = app::load_certs(&config.tls.cert_path)?;
        let key = app::load_key(&config.tls.key_path)?;

        let server_config = rustls::ServerConfig::builder()
            .with_no_client_auth()
            .with_single_cert(certs, key)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

        info!(
            service.name = %config.service.name,
            service.version = %config.service.version,
            address = %address,
            tls = true,
            "certificate service started"
        );

        app::serve_tls(
            address.parse()?,
            router,
            std::sync::Arc::new(server_config),
            Box::pin(shutdown_signal()),
        )
        .await?;
    } else {
        info!(
            service.name = %config.service.name,
            service.version = %config.service.version,
            address = %address,
            tls = false,
            "certificate service started"
        );

        let listener = TcpListener::bind(address).await?;
        axum::serve(listener, router)
            .with_graceful_shutdown(shutdown_signal())
            .await?;
    }

    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        if let Err(error) = tokio::signal::ctrl_c().await {
            tracing::error!(%error, "failed to listen for shutdown signal");
        }
    };

    #[cfg(unix)]
    let terminate = async {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
            }
            Err(error) => {
                tracing::error!(%error, "failed to listen for terminate signal");
            }
        }
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("shutdown signal received");
}
