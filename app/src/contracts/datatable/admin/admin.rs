use std::collections::BTreeMap;

use core_datatable::DataTableInput;
use core_web::datatable::{
    DataTableEmailExportRequestBase, DataTableFilterFieldDto, DataTableFilterFieldType,
    DataTableFilterOptionDto, DataTableQueryRequestBase, DataTableScopedContract,
};
use generated::models::{AdminType, AdminView};
use schemars::JsonSchema;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminDatatableQueryInput {
    #[validate(nested)]
    pub base: DataTableQueryRequestBase,
    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    #[schemars(length(min = 1, max = 120))]
    pub q: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    #[schemars(length(min = 1, max = 120))]
    pub email: Option<String>,
    #[serde(default)]
    pub admin_type: Option<AdminType>,
    #[serde(default)]
    #[validate(length(min = 1, max = 40))]
    #[schemars(length(min = 1, max = 40))]
    pub created_at_from: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 40))]
    #[schemars(length(min = 1, max = 40))]
    pub created_at_to: Option<String>,
}

impl AdminDatatableQueryInput {
    pub fn to_input(&self) -> DataTableInput {
        let mut input = self.base.to_input();
        let mut params = BTreeMap::new();

        if let Some(q) = self.q.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            params.insert("q".to_string(), q.to_string());
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
        if let Some(from) = self
            .created_at_from
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert("f-date-from-created_at".to_string(), from.to_string());
        }
        if let Some(to) = self
            .created_at_to
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert("f-date-to-created_at".to_string(), to.to_string());
        }

        input.params.extend(params);
        input
    }
}

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminDatatableEmailExportInput {
    #[validate(nested)]
    pub base: DataTableEmailExportRequestBase,
    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    #[schemars(length(min = 1, max = 120))]
    pub q: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    #[schemars(length(min = 1, max = 120))]
    pub email: Option<String>,
    #[serde(default)]
    pub admin_type: Option<AdminType>,
    #[serde(default)]
    #[validate(length(min = 1, max = 40))]
    #[schemars(length(min = 1, max = 40))]
    pub created_at_from: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 40))]
    #[schemars(length(min = 1, max = 40))]
    pub created_at_to: Option<String>,
}

impl AdminDatatableEmailExportInput {
    pub fn to_input(&self) -> DataTableInput {
        let mut input = self.base.query.to_input();
        let mut params = BTreeMap::new();

        if let Some(q) = self.q.as_deref().map(str::trim).filter(|v| !v.is_empty()) {
            params.insert("q".to_string(), q.to_string());
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
        if let Some(from) = self
            .created_at_from
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert("f-date-from-created_at".to_string(), from.to_string());
        }
        if let Some(to) = self
            .created_at_to
            .as_deref()
            .map(str::trim)
            .filter(|v| !v.is_empty())
        {
            params.insert("f-date-to-created_at".to_string(), to.to_string());
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
        "admin.admin"
    }

    fn query_to_input(&self, req: &Self::QueryRequest) -> DataTableInput {
        req.to_input()
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

    fn include_meta(&self, req: &Self::QueryRequest) -> bool {
        req.base.include_meta
    }

    fn filter_rows(&self) -> Vec<Vec<DataTableFilterFieldDto>> {
        vec![
            vec![
                DataTableFilterFieldDto {
                    field: "q".to_string(),
                    filter_key: "q".to_string(),
                    field_type: DataTableFilterFieldType::Text,
                    label: "Keyword".to_string(),
                    placeholder: Some("Search name/email".to_string()),
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
                field: "admin_type".to_string(),
                filter_key: "f-admin_type".to_string(),
                field_type: DataTableFilterFieldType::Select,
                label: "Admin Type".to_string(),
                placeholder: Some("Choose type".to_string()),
                description: None,
                options: Some(
                    AdminType::variants()
                        .iter()
                        .map(|ty| DataTableFilterOptionDto {
                            label: ty.as_str().to_string(),
                            value: ty.as_str().to_string(),
                        })
                        .collect(),
                ),
            }],
        ]
    }
}
