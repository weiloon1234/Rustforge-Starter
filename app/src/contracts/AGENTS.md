# Contracts

Request/response DTOs that define the API surface. Lives in `contracts/api/v1/` (versioned), `contracts/datatable/`, and `contracts/types/`.

## Input Structs — `#[rustforge_contract]`

Auto-injects `Debug, Clone, Deserialize, Validate, JsonSchema`. Use `#[rf(...)]` for validation rules.

```rust
use core_web::contracts::rustforge_contract;

#[rustforge_contract]
pub struct CreateArticleInput {
    #[rf(length(min = 3, max = 255))]
    #[rf(alpha_dash)]
    pub slug: String,

    #[rf(length(min = 1, max = 1000))]
    pub title: String,

    #[serde(default)]
    #[rf(email)]
    pub email: Option<String>,

    #[rf(nested)]
    pub metadata: MetadataInput,
}
```

## `#[rf(...)]` Rules

| Rule | Usage |
|------|-------|
| `length(min, max)` | `#[rf(length(min = 3, max = 64))]` |
| `range(min, max)` | `#[rf(range(min = 1, max = 100))]` |
| `email` | `#[rf(email)]` |
| `url` | `#[rf(url)]` |
| `alpha_dash` | letters, digits, `_`, `-` |
| `one_of(...)` | `#[rf(one_of("a", "b", "c"))]` |
| `none_of(...)` | `#[rf(none_of("x", "y"))]` |
| `regex(pattern)` | `#[rf(regex(pattern = r"^\d{4}$"))]` |
| `contains(pattern)` | `#[rf(contains(pattern = "@"))]` |
| `does_not_contain(pattern)` | `#[rf(does_not_contain(pattern = "banned"))]` |
| `must_match(other)` | `#[rf(must_match(other = "password_confirmation"))]` |
| `nested` | validate nested struct recursively |
| `date(format)` | `#[rf(date(format = "%Y-%m-%d"))]` |
| `phonenumber(field)` | `#[rf(phonenumber(field = "country_iso2"))]` |
| `async_unique(...)` | `#[rf(async_unique(table = "user", column = "email"))]` |
| `async_exists(...)` | `#[rf(async_exists(table = "role", column = "id"))]` |
| `async_not_exists(...)` | `#[rf(async_not_exists(table = "banned", column = "email"))]` |
| `openapi(...)` | `#[rf(openapi(description = "...", example = "..."))]` |

### Async unique with modifiers

```rust
#[rf(async_unique(
    table = "admin", column = "username",
    ignore(column = "id", field = "__target_id"),
    where_null(column = "deleted_at")
))]
```

### Update contracts with target ID for ignore

```rust
#[rustforge_contract]
pub struct UpdateArticleInput {
    #[serde(skip, default)]
    __target_id: i64,

    #[serde(default)]
    #[rf(length(min = 3, max = 255))]
    #[rf(async_unique(table = "article", column = "slug", ignore(column = "id", field = "__target_id")))]
    pub slug: Option<String>,
}

impl UpdateArticleInput {
    pub fn with_target_id(mut self, id: i64) -> Self {
        self.__target_id = id;
        self
    }
}
```

## Output Structs — manual derives (no macro)

```rust
#[derive(Debug, Clone, Serialize, JsonSchema)]
pub struct ArticleOutput {
    pub id: i64,
    pub title: String,
    #[schemars(with = "String")]
    pub created_at: time::OffsetDateTime,
}

impl From<generated::models::ArticleView> for ArticleOutput {
    fn from(v: generated::models::ArticleView) -> Self {
        Self { id: v.id, title: v.title, created_at: v.created_at }
    }
}
```

Use `#[schemars(with = "String")]` for types that don't implement `JsonSchema` (e.g. `time::OffsetDateTime`).

## Reusable String-Wrapper Types

For validation rules shared across contracts, define in `contracts/types/`:

```rust
use core_web::contracts::rustforge_string_rule_type;

rustforge_string_rule_type! {
    pub struct EmailAddress {
        #[rf(email)]
        #[rf(openapi(description = "Valid email", example = "user@example.com"))]
    }
}
```

Use as field type with `#[rf(nested)]`:
```rust
#[rf(nested)]
pub email: EmailAddress,
```
