use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ServiceConfig {
    pub name: String,
    pub version: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TelemetryConfig {
    pub log_level: String,
    pub json_logs: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TlsConfig {
    pub enabled: bool,
    pub cert_path: String,
    pub key_path: String,
    pub ca_cert_path: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NatsConfig {
    pub url: String,
}
