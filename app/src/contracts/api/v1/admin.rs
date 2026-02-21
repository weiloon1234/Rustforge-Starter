use generated::{models::AdminType, permissions::Permission};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct CreateAdminInput {
    #[validate(custom(function = "crate::validation::username::validate_username"))]
    #[validate(length(min = 3, max = 64))]
    #[schemars(length(min = 3, max = 64))]
    pub username: String,
    #[serde(default)]
    #[validate(email)]
    #[schemars(email)]
    pub email: Option<String>,
    #[validate(length(min = 1, max = 120))]
    #[schemars(length(min = 1, max = 120))]
    pub name: String,
    pub admin_type: AdminType,
    #[validate(length(min = 8, max = 128))]
    #[schemars(length(min = 8, max = 128))]
    pub password: String,
    #[serde(default)]
    pub abilities: Vec<Permission>,
}

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct UpdateAdminInput {
    #[serde(default)]
    #[validate(custom(function = "crate::validation::username::validate_username"))]
    #[validate(length(min = 3, max = 64))]
    #[schemars(length(min = 3, max = 64))]
    pub username: Option<String>,
    #[serde(default)]
    #[validate(email)]
    #[schemars(email)]
    pub email: Option<String>,
    #[serde(default)]
    #[validate(length(min = 1, max = 120))]
    #[schemars(length(min = 1, max = 120))]
    pub name: Option<String>,
    #[serde(default)]
    pub admin_type: Option<AdminType>,
    #[serde(default)]
    pub abilities: Option<Vec<Permission>>,
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
