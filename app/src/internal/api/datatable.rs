use std::sync::Arc;

use async_trait::async_trait;
use axum::http::HeaderMap;
use core_datatable::{DataTableAsyncExportManager, DataTableContext, DataTableRegistry};
use core_web::datatable::{DataTableRouteOptions, DataTableRouteState};
use core_web::openapi::ApiRouter;

use crate::internal::api::state::AppApiState;

pub fn router(state: AppApiState) -> ApiRouter {
    core_web::datatable::routes_with_prefix_and_options(
        "/datatable",
        state,
        DataTableRouteOptions {
            include_multipart_endpoints: true,
            require_bearer_auth: true,
        },
    )
}

#[async_trait]
impl DataTableRouteState for AppApiState {
    fn datatable_registry(&self) -> &Arc<DataTableRegistry> {
        &self.datatable_registry
    }

    fn datatable_async_exports(&self) -> &Arc<DataTableAsyncExportManager> {
        &self.datatable_async_exports
    }

    async fn datatable_context(&self, headers: &HeaderMap) -> DataTableContext {
        DataTableContext {
            default_per_page: self.datatable_default_per_page,
            app_timezone: self.app_timezone.clone(),
            user_timezone: core_web::utils::datatable::parse_timezone_from_headers(headers),
            actor: None,
            unknown_filter_mode: self.datatable_unknown_filter_mode,
        }
    }
}
