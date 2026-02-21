use axum::middleware::from_fn_with_state;
use core_web::openapi::{
    aide::axum::routing::get,
    ApiRouter,
};

use crate::internal::api::{datatable, state::AppApiState};

pub fn router(state: AppApiState) -> ApiRouter {
    ApiRouter::new()
        .nest("/user", user_router())
        .nest("/admin", admin_router(state))
}

fn user_router() -> ApiRouter {
    ApiRouter::new().api_route("/health", get(user_health))
}

fn admin_router(state: AppApiState) -> ApiRouter {
    ApiRouter::new()
        .api_route("/health", get(admin_health))
        .merge(datatable::router(state.clone()))
        .layer(from_fn_with_state(
            state,
            crate::internal::middleware::auth::require_admin,
        ))
}

async fn user_health() -> &'static str {
    "ok"
}

async fn admin_health() -> &'static str {
    "ok"
}
