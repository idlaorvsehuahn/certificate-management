use inventory_service::{app, config::AppConfig};
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
    
    // Build AppState and Router
    let state = app::AppState::new(config.clone(), db_pool);
    let router = inventory_service::api::routes::build_router(state.clone());

    // Spawn NATS subscription background worker task
    let service = state.inventory_service.clone();
    let nats_client_clone = nats_client.clone();
    tokio::spawn(async move {
        if let Err(e) = inventory_service::infrastructure::messaging::nats::run_nats_subscriber(nats_client_clone, service).await {
            tracing::error!(error = %e, "NATS subscriber background task failed");
        }
    });

    if config.tls.enabled {
        let certs = app::load_certs(&config.tls.cert_path)?;
        let key = app::load_key(&config.tls.key_path)?;

        // Build CA trust store for client cert verification (mTLS) - since we only allow optional client auth
        let ca_file = std::fs::File::open(&config.tls.ca_cert_path)?;
        let mut ca_reader = std::io::BufReader::new(ca_file);
        let ca_certs = rustls_pemfile::certs(&mut ca_reader)
            .collect::<Result<Vec<_>, _>>()?;

        let mut root_store = rustls::RootCertStore::empty();
        for cert in ca_certs {
            root_store.add(cert)?;
        }

        let client_verifier = rustls::server::WebPkiClientVerifier::builder(std::sync::Arc::new(root_store))
            .allow_unauthenticated()
            .build()?;

        let server_config = rustls::ServerConfig::builder()
            .with_client_cert_verifier(client_verifier)
            .with_single_cert(certs, key)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))?;

        info!(
            service.name = %config.service.name,
            service.version = %config.service.version,
            address = %address,
            tls = true,
            "inventory service started"
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
            "inventory service started"
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