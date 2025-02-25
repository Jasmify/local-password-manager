# Local Password Manager

## 日本語バージョン

Local Password Managerは、パスワードを安全に管理するためのシンプルで使いやすいアプリです。

### ライセンス

Local Password Manager (LPM)は、Jasmifyのプロプライエタリライセンスの下で提供されています。

- **商用利用**: 許可されていません。
- **再配布**: 許可されていません。
- **改変**: 許可されていません。

### 重要ファイルと取り扱いについて

#### キーファイルの生成

アプリ起動時に、以下の条件を満たす場合、新しいキーファイルが作成されます。

- 環境変数 `JASMIFY_AES_KEY` が設定されていない。
- 起動ディレクトリに `encrypted_key.hex` が存在しない。

キーファイルは、オペレーティングシステムの乱数生成機能を利用して、安全な乱数を生成します。

#### 暗号化・復号化のキーの読み込み

暗号化および復号化の際、以下の優先順位でキーを読み込みます。

1. 環境変数 `JASMIFY_AES_KEY` にキーが設定されている場合、それを使用。
2. 環境変数がない場合、`encrypted_key.hex` を読み込み。

#### `encrypted_key.hex` について

以下の操作を行うと、既存のデータを復号できなくなるため、**注意してください**。

- `encrypted_key.hex` のファイル名を変更する。
- ファイルを削除する。
- 記載内容を変更する。

`JASMIFY_AES_KEY` という名前にした理由は、他の環境変数と重複しにくくするためです。また、作成者の証として命名しました。

#### データベースについて

このアプリは SQLite を使用しています。

##### アプリ初回起動時の動作

アプリ起動時に `DB/db.sqlite` が存在しない場合、新しいデータベースが作成されます。

##### `DB/db.sqlite` の取り扱い

以下の操作を行うと、データベースを正しく読み込めなくなります。

- ファイル構成の変更
- ファイル名の変更
- ファイルの削除

#### アプリの重要ファイルについて

アプリの動作において、データベースファイル（`db.sqlite`）とキーファイル（`encrypted_key.hex`）は非常に重要な役割を果たします。これらのファイルは、アプリのデータとセキュリティを保護するために不可欠です。

- **データベースファイル（`db.sqlite`）**:

  - このファイルには、アプリで管理されるすべてのデータが保存されています。
  - ファイルを削除、名前を変更、または移動すると、アプリがデータにアクセスできなくなります。

- **キーファイル（`encrypted_key.hex`）**:
  - このファイルは、データの暗号化と復号化に使用されるキーを保持しています。
  - キーファイルを削除、名前を変更、または内容を変更すると、既存のデータを復号できなくなります。

**注意**: これらのファイルを誤って削除したり変更したりすると、アプリのデータが失われたり、アクセスできなくなる可能性があります。ファイルの取り扱いには十分注意してください。

**要するに**: これらのファイルが何をしているのか分からない場合は、削除や変更をしないでください。

#### 初期化方法

データをリセットしたい場合、以下の操作を行ってください。

- `DB` ディレクトリを削除すると、アプリ起動時に新たなデータベースが作成されます。
- `encrypted_key.hex` を削除すると、新しいキーファイルが作成されます。ただし、環境変数 `JASMIFY_AES_KEY` が設定されている場合、新たなキーファイルは作成されません。

### 開発者

- 開発者: Jasmify

## English Version

Local Password Manager is a simple and user-friendly app for securely managing passwords.

### License

Local Password Manager (LPM) is provided under Jasmify's proprietary license.

- **Commercial Use**: Not permitted.
- **Redistribution**: Not permitted.
- **Modification**: Not permitted.

### Important Files and Handling

#### Key File Generation

A new key file is created upon app startup if the following conditions are met:

- The environment variable `JASMIFY_AES_KEY` is not set.
- `encrypted_key.hex` does not exist in the startup directory.

The key file uses the operating system's random number generation to create a secure random key.

#### Encryption and Decryption Key Loading

Keys are loaded in the following priority for encryption and decryption:

1. If the environment variable `JASMIFY_AES_KEY` is set, it is used.
2. If the environment variable is not set, `encrypted_key.hex` is used.

#### About `encrypted_key.hex`

Performing the following actions will make existing data unrecoverable, **please be careful**:

- Renaming `encrypted_key.hex`.
- Deleting the file.
- Modifying the contents.

The name `JASMIFY_AES_KEY` was chosen to avoid conflicts with other environment variables and as a mark of the creator.

#### About the Database

This app uses SQLite.

##### Initial App Startup Behavior

A new database is created if `DB/db.sqlite` does not exist upon app startup.

##### Handling `DB/db.sqlite`

Performing the following actions will prevent the database from being read correctly:

- Changing the file structure
- Renaming the file
- Deleting the file

#### About Important App Files

The database file (`db.sqlite`) and key file (`encrypted_key.hex`) play a crucial role in the app's operation. These files are essential for protecting the app's data and security.

- **Database File (`db.sqlite`)**:

  - This file stores all data managed by the app.
  - Deleting, renaming, or moving the file will prevent the app from accessing the data.

- **Key File (`encrypted_key.hex`)**:
  - This file holds the key used for data encryption and decryption.
  - Deleting, renaming, or modifying the key file will make existing data unrecoverable.

**Caution**: Accidentally deleting or modifying these files may result in data loss or inaccessibility. Handle these files with care.

**In short**: If you are unsure about these files, do not delete or modify them.

#### Initialization Method

To reset the data, perform the following actions:

- Deleting the `DB` directory will create a new database upon app startup.
- Deleting `encrypted_key.hex` will create a new key file. However, if the environment variable `JASMIFY_AES_KEY` is set, a new key file will not be created.

### Developer

- Developer: Jasmify
