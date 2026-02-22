use crate::contracts::types::username::UsernameString;
use core_web::contracts::rustforge_contract;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;
use core_web::auth::AuthClientType;
use generated::models::AdminType;

#[rustforge_contract]
#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminLoginInput {
    #[rf(nested)]
    pub username: UsernameString,

    #[rf(length(min = 8, max = 128))]
    pub password: String,

    pub client_type: AuthClientType,
}

#[rustforge_contract]
#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminRefreshInput {
    pub client_type: AuthClientType,
    #[serde(default)]
    #[rf(length(min = 1, max = 256))]
    pub refresh_token: Option<String>,
}

#[rustforge_contract]
#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminLogoutInput {
    pub client_type: AuthClientType,
    #[serde(default)]
    #[rf(length(min = 1, max = 256))]
    pub refresh_token: Option<String>,
}

#[rustforge_contract]
#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminProfileUpdateInput {
    #[rf(length(min = 1, max = 120))]
    pub name: String,
    #[serde(default)]
    #[rf(email)]
    pub email: Option<String>,
}

#[rustforge_contract]
#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminPasswordUpdateInput {
    #[rf(length(min = 8, max = 128))]
    pub current_password: String,
    #[rf(length(min = 8, max = 128))]
    #[rf(must_match(other = "password_confirmation"))]
    pub password: String,
    #[rf(length(min = 8, max = 128))]
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
