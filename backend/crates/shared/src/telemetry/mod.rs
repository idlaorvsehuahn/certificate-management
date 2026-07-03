pub mod metrics;

use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::TelemetryConfig;

pub fn init(config: &TelemetryConfig) {
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(&config.log_level))
        .unwrap_or_else(|_| EnvFilter::new("info"));

    let registry = tracing_subscriber::registry().with(env_filter);

    if config.json_logs {
        registry
            .with(tracing_subscriber::fmt::layer().json())
            .init();
    } else {
        registry.with(tracing_subscriber::fmt::layer()).init();
    }
}
