mod caesar;
mod vigenere;

pub use caesar::*;
pub use vigenere::*;

pub enum EncryptionType {
    Caesar,
    Vigenere,
}

pub fn encrypt(encryption_type: EncryptionType, text: &str) -> String {
    match encryption_type {
        EncryptionType::Caesar => caesar::encrypt(text),
        EncryptionType::Vigenere => vigenere::encrypt(text),
    }
}
