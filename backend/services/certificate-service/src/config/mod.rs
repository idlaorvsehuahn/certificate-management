use serde::Deserialize;
use shared::config::{ServiceConfig, ServerConfig, DatabaseConfig, TelemetryConfig, TlsConfig, NatsConfig};

#[derive(Clone, Debug, Deserialize)]
pub struct AppConfig {
    pub service: ServiceConfig,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub telemetry: TelemetryConfig,
    pub tls: TlsConfig,
    pub nats: NatsConfig,
}

impl AppConfig {
    pub fn from_environment() -> Result<Self, ::config::ConfigError> {
        ::config::Config::builder()
            .set_default("service.name", "certificate-service")?
            .set_default("service.version", env!("CARGO_PKG_VERSION"))?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.port", 8080)?
            .set_default(
                "database.url",
                "postgres://postgres:postgres@localhost:5432/certificate_db",
            )?
            .set_default("database.max_connections", 5)?
            .set_default("telemetry.log_level", "info")?
            .set_default("telemetry.json_logs", false)?
            .set_default("tls.enabled", true)?
            .set_default("tls.cert_path", "deploy/certs/certificate-service.crt")?
            .set_default("tls.key_path", "deploy/certs/certificate-service.key")?
            .set_default("tls.ca_cert_path", "deploy/certs/ca.crt")?
            .set_default("nats.url", "nats://localhost:4222")?
            .add_source(
                ::config::Environment::with_prefix("CERTIFICATE_SERVICE")
                    .prefix_separator("__")
                    .separator("__"),
            )
            .build()?
            .try_deserialize()
    }
}
