use async_trait::async_trait;
use core_db::{
    common::{auth::hash::hash_password, sql::DbConn},
    platform::auth_subject_permissions::repo::AuthSubjectPermissionRepo,
    seeder::Seeder,
};
use uuid::Uuid;

#[derive(Debug, Default)]
pub struct AdminBootstrapSeeder;

#[async_trait]
impl Seeder for AdminBootstrapSeeder {
    async fn run(&self, db: &sqlx::PgPool) -> anyhow::Result<()> {
        if should_skip_in_env() {
            return Ok(());
        }

        let developer_id = upsert_admin(
            db,
            &env_or("SEED_ADMIN_DEVELOPER_EMAIL", "developer@example.com"),
            &env_or("SEED_ADMIN_DEVELOPER_PASSWORD", "password123"),
            "Developer",
            "developer",
        )
        .await?;

        let superadmin_id = upsert_admin(
            db,
            &env_or("SEED_ADMIN_SUPERADMIN_EMAIL", "superadmin@example.com"),
            &env_or("SEED_ADMIN_SUPERADMIN_PASSWORD", "password123"),
            "Super Admin",
            "superadmin",
        )
        .await?;

        let repo = AuthSubjectPermissionRepo::new(DbConn::pool(db));
        repo.replace("admin", developer_id, &["*".to_string()])
            .await?;

        let super_permissions = generated::permissions::Permission::all()
            .iter()
            .map(|permission| permission.as_str().to_string())
            .collect::<Vec<_>>();
        repo.replace("admin", superadmin_id, &super_permissions)
            .await?;

        Ok(())
    }

    fn name(&self) -> &str {
        "AdminBootstrapSeeder"
    }
}

fn should_skip_in_env() -> bool {
    let app_env = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".to_string())
        .trim()
        .to_ascii_lowercase();

    if app_env != "production" {
        return false;
    }

    !is_truthy(&std::env::var("SEED_ADMIN_BOOTSTRAP_IN_PROD").unwrap_or_default())
}

fn is_truthy(raw: &str) -> bool {
    matches!(
        raw.trim().to_ascii_lowercase().as_str(),
        "1" | "true" | "yes" | "on" | "y"
    )
}

fn env_or(key: &str, default: &str) -> String {
    std::env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_string())
}

async fn upsert_admin(
    db: &sqlx::PgPool,
    email: &str,
    password_plain: &str,
    name: &str,
    admin_type: &str,
) -> anyhow::Result<Uuid> {
    let password = hash_password(password_plain)?;

    let id = sqlx::query_scalar::<_, Uuid>(
        "\n        INSERT INTO admin (email, password, name, admin_type)\n        VALUES ($1, $2, $3, $4)\n        ON CONFLICT (email) DO UPDATE\n        SET\n            password = EXCLUDED.password,\n            name = EXCLUDED.name,\n            admin_type = EXCLUDED.admin_type,\n            updated_at = NOW()\n        RETURNING id\n        ",
    )
    .bind(email)
    .bind(password)
    .bind(name)
    .bind(admin_type)
    .fetch_one(db)
    .await?;

    Ok(id)
}
