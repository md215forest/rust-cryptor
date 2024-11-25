const IP: [u8; 64] = [
    58, 50, 42, 34, 26, 18, 10, 2, 60, 52, 44, 36, 28, 20, 12, 4, 62, 54, 46, 38, 30, 22, 14, 6,
    64, 56, 48, 40, 32, 24, 16, 8, 57, 49, 41, 33, 25, 17, 9, 1, 59, 51, 43, 35, 27, 19, 11, 3, 61,
    53, 45, 37, 29, 21, 13, 5, 63, 55, 47, 39, 31, 23, 15, 7,
];

const IP_INV: [u8; 64] = [
    40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22, 62, 30,
    37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11, 51, 19, 59, 27,
    34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25,
];

const DES_KEY: [[u8; 48]; 16] = [[0u8; 48]; 16];

fn init_permutation(data: &[u8; 64]) -> [u8; 64] {
    let mut result = [0u8; 64];
    for (i, &pas) in IP.iter().enumerate() {
        result[i] = data[pas as usize - 1];
    }
    result
}

fn inverse_init_permutation(data: &[u8; 64]) -> [u8; 64] {
    let mut result = [0u8; 64];
    for (i, &pas) in IP_INV.iter().enumerate() {
        result[i] = data[pas as usize - 1];
    }
    result
}

fn to_bits(text: &str) -> [u8; 64] {
    let mut result = [0u8; 64];
    for (i, c) in text.chars().enumerate() {
        for j in 0..8 {
            result[i * 8 + j] = ((c as u8) >> (7 - j)) & 1;
        }
    }
    result
}

fn to_string(bits: &[u8; 64]) -> String {
    let mut result = String::new();
    for i in 0..8 {
        let mut byte = 0;
        for j in 0..8 {
            byte |= bits[i * 8 + j] << j;
        }
        result.push(byte as char);
    }
    result
}

fn feistel_function(data: &[u8; 32], _key: &[u8; 48]) -> [u8; 32] {
    *data
}

pub fn encrypt(text: &str) -> String {
    let data = to_bits(text);
    let mut permuted_data = init_permutation(&data);
    let (left, right) = permuted_data.split_at_mut(32);
    for i in 0..16 {
        let temp_right = right.to_vec();
        let feistel_result = feistel_function(
            &right.try_into().expect("slice with incorrect length"),
            &DES_KEY[i],
        );
        for j in 0..32 {
            right[j] = left[j] ^ feistel_result[j];
        }
        left.copy_from_slice(&temp_right);
    }
    let mut combined_data = [0u8; 64];
    combined_data[..32].copy_from_slice(right);
    combined_data[32..].copy_from_slice(left);
    let encrypted_data = inverse_init_permutation(&combined_data);
    to_string(&encrypted_data)
}

pub fn decrypt(data: [u8; 64]) -> String {
    let permuted = inverse_init_permutation(&data);
    to_string(&permuted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let plaintext = "ABC";
        let encrypted = encrypt(plaintext);
        println!("{:?}", encrypted);
    }

    #[test]
    fn test_decrypt() {
        let plaintext: [u8; 64] = [
            40, 8, 48, 16, 56, 24, 64, 32, 39, 7, 47, 15, 55, 23, 63, 31, 38, 6, 46, 14, 54, 22,
            62, 30, 37, 5, 45, 13, 53, 21, 61, 29, 36, 4, 44, 12, 52, 20, 60, 28, 35, 3, 43, 11,
            51, 19, 59, 27, 34, 2, 42, 10, 50, 18, 58, 26, 33, 1, 41, 9, 49, 17, 57, 25,
        ];
        let decrypted = decrypt(plaintext);
        println!("{:?}", decrypted);
    }
}
