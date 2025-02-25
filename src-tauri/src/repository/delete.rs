use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};

pub async fn delete_account(sqlite_pool: &SqlitePool, account_ulid: &str) -> Result<()> {
    let mut tx = sqlite_pool.begin().await?;

    sqlx::query(
        r#"
        DELETE FROM accounts WHERE ulid = ?
        "#,
    )
    .bind(account_ulid)
    .execute(&mut *tx)
    .await?;

    delete_unused_category(&mut tx).await?;

    tx.commit().await?;

    Ok(())
}

async fn delete_unused_category(tx: &mut Transaction<'_, Sqlite>) -> Result<()> {
    sqlx::query(
        r#"
        DELETE FROM categories
        WHERE id NOT IN (
            SELECT DISTINCT category_id FROM account_categories
        )
        "#,
    )
    .execute(&mut **tx)
    .await?;

    Ok(())
}
