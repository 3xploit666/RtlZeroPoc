pub const ENCRYPTION_KEY: &[u8] = &[0xA5, 0xB6, 0xC7, 0xD8];
use std::string::FromUtf8Error;

pub fn decrypt_string(encrypted: &[u8], key: &[u8]) -> Result<String, FromUtf8Error> {
    let decrypted_bytes: Vec<u8> = encrypted.iter()
        .enumerate()
        .map(|(i, &b)| (b ^ key[i % key.len()]).wrapping_sub(0xA5))
        .collect();
    String::from_utf8(decrypted_bytes)
}


pub fn encrypt_string(input: &[u8], key: &[u8]) -> Vec<u8> {
    let encrypted_bytes: Vec<u8> = input.iter()
        .enumerate()
        .map(|(i, &b)| (b.wrapping_add(0xA5)) ^ key[i % key.len()])
        .collect();
    encrypted_bytes
}

/// Prints the encrypted bytes in hexadecimal format.
///
/// # Arguments
///
/// * `encrypted` - A slice of bytes that holds the encrypted data.
pub fn print_encrypted_hex(encrypted: &[u8]) {
    for byte in encrypted {
        print!("{:02X} ", byte);
    }
    println!();
}

#[macro_export]
macro_rules! obst {
    ($s:expr, $key:expr) => {{
        const fn xor_encrypt_const(input: &str, key: &[u8]) -> [u8; 256] {
            const fn xor_encrypt_recursive(input: &[u8], key: &[u8], i: usize, mut acc: [u8; 256]) -> [u8; 256] {
                if i == input.len() {
                    acc
                } else {
                    acc[i] = (input[i].wrapping_add(0xA5)) ^ key[i % key.len()];
                    xor_encrypt_recursive(input, key, i + 1, acc)
                }
            }
            xor_encrypt_recursive(input.as_bytes(), key, 0, [0; 256])
        }

        const ENCRYPTED: [u8; 256] = xor_encrypt_const($s, $key);

        RtlZeroPoc::print_encrypted_hex(&ENCRYPTED[..$s.len()]);
        decrypt_string(&ENCRYPTED[..$s.len()], $key)
    }};
}
