use crate::ctx::Context;
use axum::extract::State;

pub(crate) async fn health_endpoint(State(ctx): State<Context>) -> &'static str {
    tracing::debug!("run id: {}", ctx.execution_start_time);
    "healthy"
}
