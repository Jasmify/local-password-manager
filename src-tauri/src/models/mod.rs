use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FormData {
    pub account_name: String,
    pub identifier: String,
    pub passwords: Vec<String>,
    pub category_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    pub account_ulid: String,
    pub account_name: String,
    pub identifier_ulid: String,
    pub identifier: String,
    pub passwords: Vec<PasswordInfo>,
    pub category_name: String,
}

pub enum FormDataField {
    AccountName,
    Identifier,
    Passwords,
    CategoryName,
}

impl FormData {
    pub fn diff(&self, other: &FormData) -> Vec<FormDataField> {
        let mut differences = Vec::new();

        if self.account_name != other.account_name {
            differences.push(FormDataField::AccountName);
        }
        if self.identifier != other.identifier {
            differences.push(FormDataField::Identifier);
        }
        if self.passwords != other.passwords {
            differences.push(FormDataField::Passwords);
        }
        if self.category_name != other.category_name {
            differences.push(FormDataField::CategoryName);
        }

        differences
    }
}

impl From<AccountInfo> for FormData {
    fn from(account_info: AccountInfo) -> Self {
        FormData {
            account_name: account_info.account_name,
            identifier: account_info.identifier,
            passwords: account_info
                .passwords
                .into_iter()
                .map(|p| p.password_raw)
                .collect(),
            category_name: account_info.category_name,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PasswordData {
    pub encrypted_value: Vec<u8>,
    pub nonce: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PasswordInfo {
    pub id: u32,
    pub password_raw: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountSummary {
    pub account_ulid: String,
    pub account_name: String,
    pub identifier: String,
    pub identifier_ulid: String,
    pub category_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchCriteria {
    pub account_name: String,
    pub identifier: String,
    pub category_name: String,
}
