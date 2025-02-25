use crate::crypto;
use crate::models::FormData;
use aes_gcm::{Aes256Gcm, Key};
use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};
use ulid::Ulid;

pub async fn insert_new_account(sqlite_pool: &SqlitePool, form_data: FormData) -> Result<()> {
    let mut tx = sqlite_pool.begin().await?;

    let account_ulid = Ulid::new().to_string();
    let identifier_ulid = Ulid::new().to_string();

    insert_account(&mut tx, &account_ulid, &form_data.account_name).await?;
    insert_identifier(
        &mut tx,
        &account_ulid,
        &identifier_ulid,
        &form_data.identifier,
    )
    .await?;
    insert_category(&mut tx, &form_data.category_name).await?;
    insert_account_categories(&mut tx, &account_ulid, &form_data.category_name).await?;
    insert_passwords(&mut tx, &identifier_ulid, &form_data.passwords).await?;

    tx.commit().await?;

    Ok(())
}

async fn insert_account(
    tx: &mut Transaction<'_, Sqlite>,
    account_ulid: &str,
    account_name: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO accounts (ulid, account_name)
        VALUES (?,?)
        "#,
    )
    .bind(account_ulid)
    .bind(account_name)
    .execute(&mut **tx)
    .await?;
    Ok(())
}

async fn insert_identifier(
    tx: &mut Transaction<'_, Sqlite>,
    account_ulid: &str,
    identifier_ulid: &str,
    identifier: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        INSERT INTO identifiers (ulid, account_ulid, identifier)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(identifier_ulid)
    .bind(account_ulid)
    .bind(identifier)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn insert_category(tx: &mut Transaction<'_, Sqlite>, category_name: &str) -> Result<()> {
    // カテゴリが存在しない場合は挿入
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO categories (category_name)
        VALUES (?)
        "#,
    )
    .bind(category_name)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_account_categories(
    tx: &mut Transaction<'_, Sqlite>,
    account_ulid: &str,
    category_name: &str,
) -> Result<()> {
    // category_idを取得してaccount_categoriesに挿入
    sqlx::query(
        r#"
        INSERT INTO account_categories (account_ulid, category_id)
        SELECT ?, id FROM categories WHERE category_name = ?
        "#,
    )
    .bind(account_ulid)
    .bind(category_name)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn insert_passwords(
    tx: &mut Transaction<'_, Sqlite>,
    identifier_ulid: &str,
    passwords: &Vec<String>,
) -> Result<()> {
    let key: Key<Aes256Gcm> = crypto::get_encryption_key();

    for password in passwords {
        let (encrypted_value, nonce) = match crypto::encrypt_password(&key, password) {
            Ok((ct, n)) => (ct, n),
            Err(_) => return Err(anyhow::anyhow!("Encryption failed")),
        };

        sqlx::query(
            r#"
            INSERT INTO passwords (identifier_ulid, encrypted_value, nonce)
            VALUES (?, ?, ?)
            "#,
        )
        .bind(identifier_ulid)
        .bind(encrypted_value)
        .bind(nonce)
        .execute(&mut **tx)
        .await?;
    }

    Ok(())
}
