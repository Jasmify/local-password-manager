-- アカウント情報テーブル
CREATE TABLE accounts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ulid TEXT NOT NULL UNIQUE,
    account_name TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ID情報テーブル
CREATE TABLE identifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ulid TEXT NOT NULL UNIQUE,
    account_ulid TEXT NOT NULL,
    identifier TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(account_ulid) REFERENCES accounts(ulid) ON DELETE CASCADE
);

-- パスワード情報テーブル
CREATE TABLE passwords (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    identifier_ulid TEXT NOT NULL,
    encrypted_value BLOB NOT NULL,
    nonce BLOB NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY(identifier_ulid) REFERENCES identifiers(ulid) ON DELETE CASCADE
);

-- カテゴリ情報テーブル
CREATE TABLE categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_name TEXT NOT NULL UNIQUE
);

-- アカウントとカテゴリの関連付けテーブル
CREATE TABLE account_categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    account_ulid TEXT NOT NULL,
    category_id INTEGER NOT NULL,
    FOREIGN KEY(account_ulid) REFERENCES accounts(ulid) ON DELETE CASCADE,
    FOREIGN KEY(category_id) REFERENCES categories(id) ON DELETE CASCADE,
    UNIQUE(account_ulid, category_id)
);
