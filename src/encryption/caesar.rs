const CAESAR_SHIFT: u8 = 3;

pub fn encrypt(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (base + (c as u8 - base + CAESAR_SHIFT) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decrypt(text: &str) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (base + (c as u8 - base + 26 - CAESAR_SHIFT) % 26) as char
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
    fn test_encrypt() {
        let plaintext = "Hello, World!";
        let encrypted = encrypt(plaintext);
        assert_eq!(encrypted, "Khoor, Zruog!");
    }

    #[test]
    fn test_decrypt() {
        let encrypted = "Khoor, Zruog!";
        let decrypted = decrypt(&encrypted);
        assert_eq!(decrypted, "Hello, World!");
    }
}
