mod vigenere;

pub fn encrypt(text: &str) -> String {
    vigenere::encrypt(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let plaintext = "code";
        let encrypted = encrypt(plaintext);
        println!("{}", encrypted);
    }
}
