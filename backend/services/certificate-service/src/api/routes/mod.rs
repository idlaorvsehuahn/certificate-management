use axum::{
    Router,
    routing::{get, post},
};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
    cors::{Any, CorsLayer},
};

use crate::{
    api::handlers::{certificates, health},
    app::AppState,
};

pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))
        .route("/health", get(health::readiness))
        .route(
            "/certificates",
            get(certificates::list_certificates).post(certificates::issue_certificate),
        )
        .route("/certificates/parse", post(certificates::parse_certificate))
        .route("/certificates/import", post(certificates::import_certificate))
        .route("/certificates/{id}", get(certificates::get_certificate))
        .route("/metrics", get(shared::telemetry::metrics::metrics_handler))
        .with_state(state)
        .layer(cors)
        .layer(axum::middleware::from_fn(shared::telemetry::metrics::track_metrics))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
}
