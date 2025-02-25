use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use anyhow::Result;
use hex::{decode, encode};
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

const KEY_FILE: &str = "encrypted_key.hex";
pub const AES_KEY_ENV_VAR: &str = "JASMIFY_AES_KEY";

// Keyファイルのパスを取得
pub fn get_key_file_path() -> PathBuf {
    // const KEY_FILE: &str = "encrypted_key.bin";
    let home_dir = std::env::current_dir().expect("Cannot access the current directory");
    home_dir.join(KEY_FILE)
}

// Keyファイル作成
pub fn create_key_file(key_file_path: PathBuf) {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);

    // キーをHEXエンコードしてファイルに書き込む
    let hex_key = encode(key_bytes);
    std::fs::write(key_file_path, hex_key).expect("Failed to write key to file");
}

// Keyファイル取得
fn read_key_from_file(key_file: &str) -> Result<Vec<u8>> {
    let mut file = File::open(key_file)?;
    let mut hex_key = Vec::new();
    file.read_to_end(&mut hex_key)?;
    let key_bytes = decode(hex_key).expect("Failed to decode hex key");
    Ok(key_bytes)
}

// 暗号化キーを取得（環境変数またはファイルから取得）
pub fn get_encryption_key() -> Key<Aes256Gcm> {
    if let std::result::Result::Ok(hex_key) = env::var(AES_KEY_ENV_VAR) {
        let key_bytes = decode(hex_key).expect("Failed to decode hex key");
        Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
    } else {
        let key_file_path = get_key_file_path();

        let key_file_str = key_file_path
            .to_str()
            .expect("Failed to convert PathBuf to &str");
        let key_bytes = read_key_from_file(key_file_str).expect("Failed to read key from file");

        Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
    }
}

// パスワードを暗号化
pub fn encrypt_password(key: &Key<Aes256Gcm>, password: &str) -> Result<(String, String)> {
    let cipher = Aes256Gcm::new(key);

    // Nonce（12バイト）
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 暗号化
    let ciphertext = cipher
        .encrypt(nonce, password.as_bytes())
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok((encode(ciphertext), encode(nonce_bytes))) // HEXエンコード
}

// パスワードを復号化
pub fn decrypt_password(
    key: &Key<Aes256Gcm>,
    encrypted_value: &str,
    nonce: &str,
) -> Result<String> {
    let cipher = Aes256Gcm::new(key);

    // HEXデコード
    let ciphertext = decode(encrypted_value)?;
    let nonce_bytes = decode(nonce)?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    // 復号化
    let decrypted_bytes = cipher
        .decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| anyhow::anyhow!(e))?;

    Ok(String::from_utf8(decrypted_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_encryption_key_from_env() {
        // テスト用の環境変数を設定
        let test_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
        env::set_var(AES_KEY_ENV_VAR, test_key);

        // 関数を呼び出し、キーが正しく取得されるか確認
        let key = get_encryption_key();
        let expected_key_bytes = decode(test_key).expect("Failed to decode test hex key");
        let expected_key = Key::<Aes256Gcm>::from_slice(&expected_key_bytes).clone();

        assert_eq!(key, expected_key);
    }

    #[test]
    fn test_get_encryption_key_generated() {
        // 環境変数をクリア
        env::remove_var(AES_KEY_ENV_VAR);

        // 関数を呼び出し、キーが生成されるか確認
        let key = get_encryption_key();

        // キーの長さが32バイトであることを確認
        assert_eq!(key.as_slice().len(), 32);
    }

    #[test]
    fn test_encrypt_password() {
        // 環境変数からキーを取得
        let key = get_encryption_key();

        // テスト用のパスワード
        let password = "test_password";

        // 暗号化を実行
        let result = encrypt_password(&key, password);

        // 結果がエラーでないことを確認
        assert!(result.is_ok());

        // 結果の形式を確認
        if let Ok((ciphertext, nonce)) = result {
            assert_eq!(ciphertext.len(), (password.len() + 16) * 2); // Hexエンコードされた長さを確認
            assert_eq!(nonce.len(), 24); // Hexエンコードされた長さを確認
        }
    }

    #[test]
    fn test_decrypt_password() {
        // 環境変数からキーを取得
        let key = get_encryption_key();

        // テスト用のパスワード
        let password = "test_password";

        // 暗号化を実行
        let (encrypted_value, nonce) =
            encrypt_password(&key, password).expect("暗号化に失敗しました");

        // 復号化を実行
        let result = decrypt_password(&key, &encrypted_value, &nonce);

        // 結果がエラーでないことを確認
        assert!(result.is_ok());

        // 復号化されたパスワードが元のパスワードと一致することを確認
        assert_eq!(result.unwrap(), password);
    }
}
