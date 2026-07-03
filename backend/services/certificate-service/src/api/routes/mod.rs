use axum::{
    Router,
    routing::get,
};
use tower_http::{
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};

use crate::{
    api::handlers::{certificates, health},
    app::AppState,
};

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health/live", get(health::liveness))
        .route("/health/ready", get(health::readiness))
        .route("/health", get(health::readiness))
        .route(
            "/certificates",
            get(certificates::list_certificates).post(certificates::issue_certificate),
        )
        .route("/certificates/{id}", get(certificates::get_certificate))
        .route("/metrics", get(shared::telemetry::metrics::metrics_handler))
        .with_state(state)
        .layer(axum::middleware::from_fn(shared::telemetry::metrics::track_metrics))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
}
