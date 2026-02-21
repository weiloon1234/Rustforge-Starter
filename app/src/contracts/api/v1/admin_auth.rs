use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;
use core_web::auth::AuthClientType;
use generated::models::AdminType;

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminLoginInput {
    #[validate(custom(function = "crate::validation::username::validate_username"))]
    #[validate(length(min = 3, max = 64))]
    #[schemars(length(min = 3, max = 64))]
    pub username: String,

    #[validate(length(min = 8, max = 128))]
    #[schemars(length(min = 8, max = 128))]
    pub password: String,

    pub client_type: AuthClientType,
}

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminRefreshInput {
    pub client_type: AuthClientType,
    #[serde(default)]
    #[validate(length(min = 1, max = 256))]
    #[schemars(length(min = 1, max = 256))]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminLogoutInput {
    pub client_type: AuthClientType,
    #[serde(default)]
    #[validate(length(min = 1, max = 256))]
    #[schemars(length(min = 1, max = 256))]
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminProfileUpdateInput {
    #[validate(length(min = 1, max = 120))]
    #[schemars(length(min = 1, max = 120))]
    pub name: String,
    #[serde(default)]
    #[validate(email)]
    #[schemars(email)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminPasswordUpdateInput {
    #[validate(length(min = 8, max = 128))]
    #[schemars(length(min = 8, max = 128))]
    pub current_password: String,
    #[validate(length(min = 8, max = 128))]
    #[validate(must_match(other = "password_confirmation"))]
    #[schemars(length(min = 8, max = 128))]
    pub password: String,
    #[validate(length(min = 8, max = 128))]
    #[schemars(length(min = 8, max = 128))]
    pub password_confirmation: String,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminAuthOutput {
    pub token_type: String,
    pub access_token: String,
    #[schemars(with = "Option<String>")]
    pub access_expires_at: Option<time::OffsetDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminMeOutput {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub name: String,
    pub admin_type: AdminType,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminProfileUpdateOutput {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub name: String,
    pub admin_type: AdminType,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminPasswordUpdateOutput {
    pub updated: bool,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminLogoutOutput {
    pub revoked: bool,
}
