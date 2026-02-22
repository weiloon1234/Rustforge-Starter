use core_web::contracts::rustforge_string_rule_type;

rustforge_string_rule_type! {
    /// Lowercase username used for admin authentication and admin CRUD inputs.
    pub struct UsernameString {
        #[validate(custom(function = "crate::validation::username::validate_username"))]
        #[rf(length(min = 3, max = 64))]
        #[rf(rule = "alpha_dash")]
        #[rf(openapi_description = "Lowercase username using letters, numbers, underscore (_), and hyphen (-).")]
        #[rf(openapi_example = "admin_user")]
    }
}
