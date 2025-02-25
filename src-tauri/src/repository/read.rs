use crate::crypto::{decrypt_password, get_encryption_key};
use crate::models::{AccountSummary, PasswordInfo, SearchCriteria};
use anyhow::Result;
use sqlx::{Row, SqlitePool};

pub async fn get_account_summary(sqlite_pool: &SqlitePool) -> Result<Vec<AccountSummary>> {
    let accounts_rows = sqlx::query(
        r#"
        SELECT 
            a.ulid AS account_ulid,
            a.account_name,
            i.ulid AS identifier_ulid,
            i.identifier,
            c.category_name
        FROM 
            accounts a
        LEFT JOIN 
            identifiers i ON a.ulid = i.account_ulid
        LEFT JOIN 
            account_categories ac ON a.ulid = ac.account_ulid
        LEFT JOIN 
            categories c ON ac.category_id = c.id;
        "#,
    )
    .fetch_all(sqlite_pool)
    .await?;

    let mut account_summary_vec = Vec::new();

    for row in accounts_rows {
        let account_ulid: String = row.try_get("account_ulid")?;
        let account_name: String = row.try_get("account_name")?;
        let identifier: String = row.try_get("identifier")?;
        let identifier_ulid: String = row.try_get("identifier_ulid")?;
        let category_name: String = row.try_get("category_name")?;

        let account_summary = AccountSummary {
            account_ulid,
            account_name,
            identifier_ulid,
            identifier,
            category_name,
        };

        account_summary_vec.push(account_summary);
    }

    Ok(account_summary_vec)
}

pub async fn get_password_info(
    sqlite_pool: &SqlitePool,
    identifier_ulid: String,
) -> Result<Vec<PasswordInfo>> {
    let passwords_rows = sqlx::query(
        r#"
        SELECT 
            id,
            encrypted_value,
            nonce
        FROM 
            passwords
        WHERE 
            identifier_ulid = ?;
        "#,
    )
    .bind(&identifier_ulid)
    .fetch_all(sqlite_pool)
    .await?;

    let mut passwords_vec = Vec::new();

    for password in passwords_rows {
        let id: u32 = password.try_get("id")?;
        let encrypted_value: String = password.try_get("encrypted_value")?;
        let nonce: String = password.try_get("nonce")?;

        let key = get_encryption_key();
        let password_raw = decrypt_password(&key, &encrypted_value, &nonce)?;

        let password_info = PasswordInfo { id, password_raw };
        passwords_vec.push(password_info);
    }

    Ok(passwords_vec)
}

pub async fn get_search_results(
    sqlite_pool: &SqlitePool,
    search_criteria: SearchCriteria,
) -> Result<Vec<AccountSummary>> {
    let (sql, bindings) = build_filter_conditions(&search_criteria);

    let mut query = sqlx::query(&sql);
    for binding in &bindings {
        query = query.bind(binding);
    }

    let rows = query.fetch_all(sqlite_pool).await?;

    let mut search_results = Vec::<AccountSummary>::new();

    for row in rows {
        let account_ulid: String = row.try_get("account_ulid")?;
        let account_name: String = row.try_get("account_name")?;
        let identifier_ulid: String = row.try_get("identifier_ulid")?;
        let identifier: String = row.try_get("identifier")?;
        let category_name: String = row.try_get("category_name")?;

        let result = AccountSummary {
            account_ulid,
            account_name,
            identifier_ulid,
            identifier,
            category_name,
        };

        search_results.push(result);
    }

    Ok(search_results)
}

fn build_filter_conditions(criteria: &SearchCriteria) -> (String, Vec<String>) {
    let mut query = String::from(
        "SELECT 
            accounts.ulid AS account_ulid, 
            accounts.account_name, 
            identifiers.ulid AS identifier_ulid, 
            identifiers.identifier, 
            categories.category_name
        FROM accounts
        LEFT JOIN identifiers ON accounts.ulid = identifiers.account_ulid
        LEFT JOIN account_categories ON accounts.ulid = account_categories.account_ulid
        LEFT JOIN categories ON account_categories.category_id = categories.id
        WHERE 1=1",
    );

    let mut bindings = Vec::new();

    if !criteria.account_name.is_empty() {
        query.push_str(" AND accounts.account_name LIKE ?");
        bindings.push(format!("%{}%", &criteria.account_name));
    }
    if !criteria.identifier.is_empty() {
        query.push_str(" AND identifiers.identifier LIKE ?");
        bindings.push(format!("%{}%", &criteria.identifier));
    }
    if !criteria.category_name.is_empty() {
        query.push_str(" AND categories.category_name LIKE ?");
        bindings.push(format!("%{}%", &criteria.category_name));
    }

    (query, bindings)
}
