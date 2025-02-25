use crate::crypto;
use crate::models::{AccountInfo, FormData, FormDataField};
use crate::repository::insert::insert_category;
use aes_gcm::{Aes256Gcm, Key};
use anyhow::Result;
use sqlx::{Sqlite, SqlitePool, Transaction};

pub async fn update_account_info(
    sqlite_pool: &SqlitePool,
    form_data: FormData,
    account_info: AccountInfo,
) -> Result<()> {
    let mut tx = sqlite_pool.begin().await?;

    let old_form_data: FormData = account_info.clone().into();
    let differences = form_data.diff(&old_form_data);

    match differences.as_slice() {
        [] => {
            // 何も変更がない場合の処理
        }
        _ => {
            for difference in &differences {
                match difference {
                    FormDataField::AccountName => {
                        // account_nameが変更された場合の処理
                        update_account(
                            &mut tx,
                            &account_info.account_ulid,
                            &form_data.account_name,
                        )
                        .await?;
                    }
                    FormDataField::Identifier => {
                        // identifierが変更された場合の処理
                        update_identifier(
                            &mut tx,
                            &account_info.identifier_ulid,
                            &form_data.identifier,
                        )
                        .await?;
                    }
                    FormDataField::Passwords => {
                        // passwordsが変更された場合の処理
                        let key: Key<Aes256Gcm> = crypto::get_encryption_key();
                        update_passwords(&mut tx, &key, &form_data, &account_info).await?;
                    }
                    FormDataField::CategoryName => {
                        // category_nameが変更された場合の処理
                        insert_category(&mut tx, &form_data.category_name).await?;
                        update_category(
                            &mut tx,
                            &account_info.account_ulid,
                            &form_data.category_name,
                        )
                        .await?;
                    }
                }
            }
        }
    }

    tx.commit().await?;

    Ok(())
}

async fn update_account(
    tx: &mut Transaction<'_, Sqlite>,
    account_ulid: &str,
    new_account_name: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE accounts
        SET account_name = ?, updated_at = CURRENT_TIMESTAMP
        WHERE ulid = ?
        "#,
    )
    .bind(new_account_name)
    .bind(account_ulid)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn update_identifier(
    tx: &mut Transaction<'_, Sqlite>,
    identifier_ulid: &str,
    new_identifier: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE identifiers
        SET identifier = ?, updated_at = CURRENT_TIMESTAMP
        WHERE ulid = ?
        "#,
    )
    .bind(new_identifier)
    .bind(identifier_ulid)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn update_category(
    tx: &mut Transaction<'_, Sqlite>,
    account_ulid: &str,
    new_category_name: &str,
) -> Result<()> {
    sqlx::query(
        r#"
        UPDATE account_categories
        SET category_id = (SELECT id FROM categories WHERE category_name = ?)
        WHERE account_ulid = ?
        "#,
    )
    .bind(new_category_name)
    .bind(account_ulid)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

async fn update_passwords(
    tx: &mut Transaction<'_, Sqlite>,
    key: &Key<Aes256Gcm>,
    form_data: &FormData,
    account_info: &AccountInfo,
) -> Result<()> {
    update_existing_passwords(tx, key, form_data, account_info).await?;
    insert_new_passwords(tx, key, form_data, account_info).await?;
    delete_old_passwords(tx, form_data, account_info).await?;

    Ok(())
}

async fn update_existing_passwords(
    tx: &mut Transaction<'_, Sqlite>,
    key: &Key<Aes256Gcm>,
    form_data: &FormData,
    account_info: &AccountInfo,
) -> Result<()> {
    for (new_password, old_password_info) in form_data.passwords.iter().zip(&account_info.passwords)
    {
        if new_password != &old_password_info.password_raw {
            let (encrypted_value, nonce) = crypto::encrypt_password(key, new_password)?;
            sqlx::query(
                r#"
                UPDATE passwords
                SET encrypted_value = ?, nonce = ?, updated_at = CURRENT_TIMESTAMP
                WHERE id = ?
                "#,
            )
            .bind(encrypted_value)
            .bind(nonce)
            .bind(old_password_info.id)
            .execute(&mut **tx)
            .await?;
        }
    }
    Ok(())
}

async fn insert_new_passwords(
    tx: &mut Transaction<'_, Sqlite>,
    key: &Key<Aes256Gcm>,
    form_data: &FormData,
    account_info: &AccountInfo,
) -> Result<()> {
    let new_len = form_data.passwords.len();
    let old_len = account_info.passwords.len();

    if new_len > old_len {
        for new_password in &form_data.passwords[old_len..] {
            let (encrypted_value, nonce) = crypto::encrypt_password(key, new_password)?;
            sqlx::query(
                r#"
                INSERT INTO passwords (identifier_ulid, encrypted_value, nonce)
                VALUES (?, ?, ?)
                "#,
            )
            .bind(&account_info.identifier_ulid)
            .bind(encrypted_value)
            .bind(nonce)
            .execute(&mut **tx)
            .await?;
        }
    }
    Ok(())
}

async fn delete_old_passwords(
    tx: &mut Transaction<'_, Sqlite>,
    form_data: &FormData,
    account_info: &AccountInfo,
) -> Result<()> {
    let new_len = form_data.passwords.len();
    let old_len = account_info.passwords.len();

    if old_len > new_len {
        for old_password_info in &account_info.passwords[new_len..] {
            sqlx::query(
                r#"
                DELETE FROM passwords
                WHERE id = ?
                "#,
            )
            .bind(old_password_info.id)
            .execute(&mut **tx)
            .await?;
        }
    }
    Ok(())
}
