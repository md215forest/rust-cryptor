static VIGENERE_KEY: &str = "arm";

pub fn create_vigenere_spuare() -> Vec<Vec<char>> {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut vigenere_square = Vec::new();
    for i in 0..26 {
        let start = ALPHABET[i..].to_string();
        let end = ALPHABET[..i].to_string();
        let row = format!("{}{}", start, end);
        vigenere_square.push(row.chars().collect());
    }
    vigenere_square
}

fn get_key(index: usize) -> char {
    let key_len = VIGENERE_KEY.len();
    let key_index = index % key_len;
    VIGENERE_KEY.chars().nth(key_index).unwrap()
}

pub fn encrypt(text: &str) -> String {
    let vigenere_square = create_vigenere_spuare();
    let mut encoded = String::new();
    let mut key_index = 0;
    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            let is_lower = c.is_ascii_lowercase();
            let lower_c = c.to_ascii_lowercase();
            let key = get_key(key_index);
            let code =
                vigenere_square[(key as u8 - b'a') as usize][(lower_c as u8 - b'a') as usize];
            let encoded_char = if is_lower {
                code
            } else {
                code.to_uppercase().to_string().chars().next().unwrap()
            };
            encoded.push(encoded_char);
            key_index += 1;
        } else {
            encoded.push(c);
        }
    }
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vigenere_spuare() {
        let vigenere_square = create_vigenere_spuare();
        println!("{}, {}", vigenere_square[0][0], vigenere_square[0][1]);
    }

    #[test]
    fn test_encrypt() {
        let plaintext = "cOde";
        let encrypted = encrypt(plaintext);
        println!("{}", encrypted);
    }

    #[test]
    fn test_get_key() {
        let key = get_key(0);
        assert_eq!(key, 'a');
        let key = get_key(1);
        assert_eq!(key, 'r');
        let key = get_key(2);
        assert_eq!(key, 'm');
        let key = get_key(3);
        assert_eq!(key, 'a');
        let key = get_key(91);
        assert_eq!(key, 'r');
        let key = get_key(110);
        assert_eq!(key, 'm');
    }
}
