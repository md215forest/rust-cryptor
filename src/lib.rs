mod encryption;

pub use crate::encryption::*;

pub fn encrypt(text: &str) -> String {
    encryption::encrypt(EncryptionType::Vigenere, text)
}
