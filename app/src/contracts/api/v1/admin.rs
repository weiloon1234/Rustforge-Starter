use crate::contracts::types::username::UsernameString;
use core_web::contracts::rustforge_contract;
use generated::{models::AdminType, permissions::Permission};
use schemars::JsonSchema;
use serde::Serialize;
use validator::Validate;

#[rustforge_contract]
pub struct CreateAdminInput {
    #[rf(nested)]
    #[rf(async_unique(table = "admin", column = "username"))]
    pub username: UsernameString,
    #[serde(default)]
    #[rf(email)]
    pub email: Option<String>,
    #[rf(length(min = 1, max = 120))]
    pub name: String,
    pub admin_type: AdminType,
    #[rf(length(min = 8, max = 128))]
    pub password: String,
    #[serde(default)]
    pub abilities: Vec<Permission>,
}

#[rustforge_contract]
pub struct UpdateAdminInput {
    #[serde(skip, default)]
    __target_id: i64,
    #[serde(default)]
    #[rf(nested)]
    #[rf(async_unique(
        table = "admin",
        column = "username",
        ignore(column = "id", field = "__target_id")
    ))]
    pub username: Option<UsernameString>,
    #[serde(default)]
    #[rf(email)]
    pub email: Option<String>,
    #[serde(default)]
    #[rf(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[serde(default)]
    pub admin_type: Option<AdminType>,
    #[serde(default)]
    pub abilities: Option<Vec<Permission>>,
}

impl UpdateAdminInput {
    pub fn with_target_id(mut self, id: i64) -> Self {
        self.__target_id = id;
        self
    }
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminOutput {
    pub id: i64,
    pub username: String,
    pub email: Option<String>,
    pub name: String,
    pub admin_type: AdminType,
    #[serde(default)]
    pub abilities: Vec<String>,
    #[schemars(with = "String")]
    pub created_at: time::OffsetDateTime,
    #[schemars(with = "String")]
    pub updated_at: time::OffsetDateTime,
}

impl From<generated::models::AdminView> for AdminOutput {
    fn from(value: generated::models::AdminView) -> Self {
        let abilities = value
            .abilities
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item.as_str().map(ToString::to_string))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        Self {
            id: value.id,
            username: value.username,
            email: value.email,
            name: value.name,
            admin_type: value.admin_type,
            abilities,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminDeleteOutput {
    pub deleted: bool,
}
