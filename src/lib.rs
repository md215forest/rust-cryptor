pub fn encrypt(text: &str, shift: u8) -> String {
    text.chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (first + (c as u8 - first + shift) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn decrypt(text: &str, shift: u8) -> String {
    encrypt(text, 26 - shift)
}

#[cfg(test)]
mod tests {
    use super::*;
    static SHIFT_COUNT: u8 = 3;

    #[test]
    fn test_encrypt() {
        let plaintext = "Hello, World!";
        let encrypted = encrypt(plaintext, SHIFT_COUNT);
        assert_eq!(encrypted, "Khoor, Zruog!");
    }

    #[test]
    fn test_decrypt() {
        let encrypted = "Khoor, Zruog!";
        let decrypted = decrypt(&encrypted, SHIFT_COUNT);
        assert_eq!(decrypted, "Hello, World!");
    }
}
