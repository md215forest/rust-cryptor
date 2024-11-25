mod caesar;
mod des;
mod vigenere;

pub enum EncryptionType {
    Caesar,
    Vigenere,
    DES,
}

pub fn encrypt(encryption_type: EncryptionType, text: &str) -> String {
    match encryption_type {
        EncryptionType::Caesar => caesar::encrypt(text),
        EncryptionType::Vigenere => vigenere::encrypt(text),
        EncryptionType::DES => des::encrypt(text),
    }
}
