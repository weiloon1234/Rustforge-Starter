use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, JsonSchema)]
pub struct AdminLoginInput {
    #[validate(length(min = 3, max = 64))]
    #[schemars(length(min = 3, max = 64))]
    pub username: String,

    #[validate(length(min = 8, max = 128))]
    #[schemars(length(min = 8, max = 128))]
    pub password: String,
}

#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct AdminLoginOutput {
    pub access_token: String,
}
