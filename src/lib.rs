mod vigenere;

pub fn encrypt(text: &str) -> String {
    vigenere::encrypt(text)
}
