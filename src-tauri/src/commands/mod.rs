use tauri::async_runtime::block_on;
use tauri::State;

use crate::{
    models::{self, AccountInfo, AccountSummary, FormData, PasswordInfo, SearchCriteria},
    repository,
};

#[tauri::command]
pub fn insert_form_data(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    form_data: models::FormData,
) -> Result<(), String> {
    if let Err(e) = block_on(repository::insert::insert_new_account(
        &sqlite_pool,
        form_data,
    )) {
        return Err(e.to_string());
    };

    Ok(())
}

#[tauri::command]
pub fn get_account_summary(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
) -> Result<Vec<AccountSummary>, String> {
    let summary = match block_on(repository::read::get_account_summary(&sqlite_pool)) {
        Ok(data) => data,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let account_summary: Vec<AccountSummary> = summary
        .into_iter()
        .map(|data| AccountSummary {
            account_ulid: data.account_ulid,
            account_name: data.account_name,
            identifier: data.identifier,
            identifier_ulid: data.identifier_ulid,
            category_name: data.category_name,
        })
        .collect();

    Ok(account_summary)
}

#[tauri::command]
pub fn get_search_results(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    search_criteria: SearchCriteria,
) -> Result<Vec<AccountSummary>, String> {
    let summary = match block_on(repository::read::get_search_results(
        &sqlite_pool,
        search_criteria,
    )) {
        Ok(data) => data,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    let search_results: Vec<AccountSummary> = summary
        .into_iter()
        .map(|data| AccountSummary {
            account_ulid: data.account_ulid,
            account_name: data.account_name,
            identifier: data.identifier,
            identifier_ulid: data.identifier_ulid,
            category_name: data.category_name,
        })
        .collect();

    Ok(search_results)
}

#[tauri::command]
pub fn get_password_info(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    identifier_ulid: String,
) -> Result<Vec<PasswordInfo>, String> {
    let password_info = match block_on(repository::read::get_password_info(
        &sqlite_pool,
        identifier_ulid,
    )) {
        Ok(data) => data,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    Ok(password_info)
}

#[tauri::command]
pub fn update_account_info(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    form_data: FormData,
    account_info: AccountInfo,
) -> Result<(), String> {
    if let Err(e) = block_on(repository::update::update_account_info(
        &sqlite_pool,
        form_data,
        account_info,
    )) {
        return Err(e.to_string());
    }

    Ok(())
}

#[tauri::command]
pub fn delete_account(
    sqlite_pool: State<'_, sqlx::SqlitePool>,
    account_ulid: String,
) -> Result<(), String> {
    if let Err(e) = block_on(repository::delete::delete_account(
        &sqlite_pool,
        &account_ulid,
    )) {
        return Err(e.to_string());
    }

    Ok(())
}
