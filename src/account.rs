use sqlx::PgPool;

/// Creates a root user if it does not already exist.
pub async fn create_root_account(
    name: &str,
    email: &str,
    password: &str,
    pool: &PgPool
) -> Result<(), anyhow::Error> {

    // Quits if root account already exists
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM account WHERE role='root'")
        .fetch_one(pool)
        .await?;
    let root_account_exists = row.0 != 0;
    if root_account_exists {
        return Ok(());
    }

    // Creates root account
    sqlx::query("INSERT INTO account (name, email, password, role) VALUES ($1, $2, $3, 'root')")
        .bind(name)
        .bind(email)
        .bind(password)
        .execute(pool)
        .await?;
    Ok(())
}