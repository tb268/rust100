#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Encrypt,
    Decrypt,
}

/// シーザー暗号でテキストを暗号化または復号化します
/// 
/// # 引数
/// - `text`: 処理するテキスト（アルファベットのみが処理されます）
/// - `shift`: シフト数（0-25の範囲）
/// - `operation`: 暗号化（Encrypt）または復号化（Decrypt）
/// 
/// # 返り値
/// 処理されたテキストを返します。アルファベット以外の文字はそのまま保持されます。
pub fn caesar_cipher(text: &str, shift: u8, operation: Operation) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let offset = (c as u8) - base;
                
                let new_offset = match operation {
                    Operation::Encrypt => (offset + shift) % 26,
                    Operation::Decrypt => (offset + 26 - shift) % 26,
                };
                
                (base + new_offset) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_uppercase() {
        let result = caesar_cipher("HELLO", 3, Operation::Encrypt);
        assert_eq!(result, "KHOOR");
    }

    #[test]
    fn test_encrypt_lowercase() {
        let result = caesar_cipher("hello", 3, Operation::Encrypt);
        assert_eq!(result, "khoor");
    }

    #[test]
    fn test_encrypt_mixed_case() {
        let result = caesar_cipher("Hello World", 3, Operation::Encrypt);
        assert_eq!(result, "Khoor Zruog");
    }

    #[test]
    fn test_decrypt_uppercase() {
        let result = caesar_cipher("KHOOR", 3, Operation::Decrypt);
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_decrypt_lowercase() {
        let result = caesar_cipher("khoor", 3, Operation::Decrypt);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_decrypt_mixed_case() {
        let result = caesar_cipher("Khoor Zruog", 3, Operation::Decrypt);
        assert_eq!(result, "Hello World");
    }

    #[test]
    fn test_with_numbers_and_symbols() {
        let result = caesar_cipher("Hello123!@#", 3, Operation::Encrypt);
        assert_eq!(result, "Khoor123!@#");
    }

    #[test]
    fn test_roundtrip() {
        let original = "The Quick Brown Fox";
        let encrypted = caesar_cipher(original, 7, Operation::Encrypt);
        let decrypted = caesar_cipher(&encrypted, 7, Operation::Decrypt);
        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_shift_25() {
        let result = caesar_cipher("ABC", 25, Operation::Encrypt);
        assert_eq!(result, "ZAB");
    }

    #[test]
    fn test_shift_0() {
        let result = caesar_cipher("Hello", 0, Operation::Encrypt);
        assert_eq!(result, "Hello");
    }
}

