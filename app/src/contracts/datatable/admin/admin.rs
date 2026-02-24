use std::collections::BTreeMap;

use core_datatable::DataTableInput;
use core_web::datatable::{
    DataTableEmailExportRequestBase, DataTableFilterFieldDto, DataTableFilterFieldType,
    DataTableQueryRequestBase, DataTableQueryRequestContract, DataTableScopedContract,
};
use core_web::contracts::rustforge_contract;
use generated::models::{AdminType, AdminView};
use ts_rs::TS;
use validator::Validate;

#[rustforge_contract]
#[derive(TS)]
#[ts(export, export_to = "admin/types/")]
pub struct AdminDatatableQueryInput {
    #[serde(default)]
    #[rf(nested)]
    #[ts(type = "DataTableQueryRequestBase")]
    pub base: DataTableQueryRequestBase,
    #[serde(default)]
    #[rf(length(min = 1, max = 120))]
    pub q: Option<String>,
    #[serde(default)]
    #[rf(length(min = 3, max = 64))]
    #[rf(alpha_dash)]
    pub username: Option<String>,
    #[serde(default)]
    #[rf(length(min = 1, max = 120))]
    pub email: Option<String>,
    #[serde(default)]
    #[ts(type = "AdminType | null")]
    pub admin_type: Option<AdminType>,
}

impl AdminDatatableQueryInput {
    pub fn to_input(&self) -> DataTableInput {
        let mut input = self.base.to_input();
        let mut params = BTreeMap::new();

        if let Some(q) = self.q.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            params.insert("q".to_string(), q.to_string());
        }
        if let Some(username) = self
            .username
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert(
                "f-like-username".to_string(),
                username.to_ascii_lowercase(),
            );
        }
        if let Some(email) = self
            .email
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert("f-like-email".to_string(), email.to_string());
        }
        if let Some(admin_type) = self.admin_type {
            params.insert("f-admin_type".to_string(), admin_type.as_str().to_string());
        }

        input.params.extend(params);
        input
    }
}

impl DataTableQueryRequestContract for AdminDatatableQueryInput {
    fn query_base(&self) -> &DataTableQueryRequestBase {
        &self.base
    }

    fn datatable_query_to_input(&self) -> DataTableInput {
        self.to_input()
    }
}

#[rustforge_contract]
#[derive(TS)]
#[ts(export, export_to = "admin/types/")]
pub struct AdminDatatableEmailExportInput {
    #[rf(nested)]
    #[ts(type = "DataTableEmailExportRequestBase")]
    pub base: DataTableEmailExportRequestBase,
    #[serde(default)]
    #[rf(length(min = 1, max = 120))]
    pub q: Option<String>,
    #[serde(default)]
    #[rf(length(min = 3, max = 64))]
    #[rf(alpha_dash)]
    pub username: Option<String>,
    #[serde(default)]
    #[rf(length(min = 1, max = 120))]
    pub email: Option<String>,
    #[serde(default)]
    #[ts(type = "AdminType | null")]
    pub admin_type: Option<AdminType>,
}

impl AdminDatatableEmailExportInput {
    pub fn to_input(&self) -> DataTableInput {
        let mut input = self.base.query.to_input();
        let mut params = BTreeMap::new();

        if let Some(q) = self.q.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            params.insert("q".to_string(), q.to_string());
        }
        if let Some(username) = self
            .username
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert(
                "f-like-username".to_string(),
                username.to_ascii_lowercase(),
            );
        }
        if let Some(email) = self
            .email
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert("f-like-email".to_string(), email.to_string());
        }
        if let Some(admin_type) = self.admin_type {
            params.insert("f-admin_type".to_string(), admin_type.as_str().to_string());
        }

        input.params.extend(params);
        input.export_file_name = self.base.export_file_name.clone();
        input
    }
}

#[derive(Debug, Clone, Default)]
pub struct AdminAdminDataTableContract;

impl DataTableScopedContract for AdminAdminDataTableContract {
    type QueryRequest = AdminDatatableQueryInput;
    type EmailRequest = AdminDatatableEmailExportInput;
    type Row = AdminView;

    fn scoped_key(&self) -> &'static str {
        "admin.account"
    }

    fn openapi_tag(&self) -> &'static str {
        "Admin Account"
    }

    fn email_to_input(&self, req: &Self::EmailRequest) -> DataTableInput {
        req.to_input()
    }

    fn email_recipients(&self, req: &Self::EmailRequest) -> Vec<String> {
        req.base.recipients.clone()
    }

    fn email_subject(&self, req: &Self::EmailRequest) -> Option<String> {
        req.base.subject.clone()
    }

    fn export_file_name(&self, req: &Self::EmailRequest) -> Option<String> {
        req.base.export_file_name.clone()
    }

    fn filter_rows(&self) -> Vec<Vec<DataTableFilterFieldDto>> {
        vec![
            vec![
                DataTableFilterFieldDto {
                    field: "q".to_string(),
                    filter_key: "q".to_string(),
                    field_type: DataTableFilterFieldType::Text,
                    label: "Keyword".to_string(),
                    placeholder: Some("Search name/username/email".to_string()),
                    description: None,
                    options: None,
                },
                DataTableFilterFieldDto {
                    field: "email".to_string(),
                    filter_key: "f-like-email".to_string(),
                    field_type: DataTableFilterFieldType::Text,
                    label: "Email".to_string(),
                    placeholder: Some("Contains".to_string()),
                    description: None,
                    options: None,
                },
            ],
            vec![DataTableFilterFieldDto {
                field: "username".to_string(),
                filter_key: "f-like-username".to_string(),
                field_type: DataTableFilterFieldType::Text,
                label: "Username".to_string(),
                placeholder: Some("Contains".to_string()),
                description: None,
                options: None,
            }],
            vec![DataTableFilterFieldDto {
                field: "admin_type".to_string(),
                filter_key: "f-admin_type".to_string(),
                field_type: DataTableFilterFieldType::Select,
                label: "Admin Type".to_string(),
                placeholder: Some("Choose type".to_string()),
                description: None,
                options: Some(AdminType::datatable_filter_options()),
            }],
        ]
    }
}
