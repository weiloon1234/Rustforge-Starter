pub mod datatable;
pub mod state;
pub mod v1;

use std::sync::Arc;

use axum::{routing::get as axum_get, Json, Router};
use bootstrap::boot::BootContext;
use core_web::openapi::{
    aide::openapi::{Info, OpenApi},
    ApiRouter,
};

use state::AppApiState;

pub async fn build_router(ctx: BootContext) -> anyhow::Result<Router> {
    let app_state = AppApiState::new(&ctx)?;

    let api_router = ApiRouter::new().nest("/api/v1", v1::router(app_state));

    let mut api = OpenApi::default();
    api.info = Info {
        title: "starter-api".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        ..Default::default()
    };

    let mut router =
        api_router.finish_api_with(&mut api, core_web::openapi::with_bearer_auth_scheme);

    if ctx.settings.app.enable_openapi_docs {
        let openapi_json_path = ctx.settings.app.openapi_json_path.clone();
        let openapi = Arc::new(api);

        router = router.route(
            openapi_json_path.as_str(),
            axum_get({
                let openapi = openapi.clone();
                move || {
                    let openapi = openapi.clone();
                    async move { Json((*openapi).clone()) }
                }
            }),
        );
    }

    let public_path = core_web::static_assets::public_path_from_env();
    if let Some(static_router) = core_web::static_assets::static_assets_router(&public_path) {
        router = router.merge(static_router);
    } else {
        router = router.route("/", axum_get(root));
    }

    Ok(router)
}

async fn root() -> &'static str {
    "ok"
}
