// App-level datatable hooks for Admin.
// Generated once by db-gen; safe to edit.

use core_datatable::DataTableRegistry;
use generated::models::{AdminDataTable, AdminDataTableConfig, AdminDataTableHooks};

#[derive(Default, Clone)]
pub struct AdminDataTableAppHooks;

impl AdminDataTableHooks for AdminDataTableAppHooks {
    // Override scope/authorize/filters/mappings when needed.
}

pub type AppAdminDataTable = AdminDataTable<AdminDataTableAppHooks>;

pub fn app_admin_datatable(db: sqlx::PgPool) -> AppAdminDataTable {
    AdminDataTable::new(db).with_hooks(AdminDataTableAppHooks::default())
}

pub fn app_admin_datatable_with_config(
    db: sqlx::PgPool,
    config: AdminDataTableConfig,
) -> AppAdminDataTable {
    AdminDataTable::new(db).with_hooks(AdminDataTableAppHooks::default()).with_config(config)
}

pub fn register_admin_datatable(registry: &mut DataTableRegistry, db: sqlx::PgPool) {
    registry.register(app_admin_datatable(db));
}
