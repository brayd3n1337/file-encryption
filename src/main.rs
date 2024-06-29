mod aes_keypair;

extern crate aes;
extern crate block_modes;
extern crate block_padding;
extern crate rand;

use std::path::Path;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::RngCore;
use crate::aes_keypair::{AESKeypair, AESKeypairImpl};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn encrypt_message(message: &str, key: &[u8; 32], iv: &[u8; 16]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();

    return cipher.encrypt_vec(message.as_bytes());
}

fn decrypt_message(ciphertext: &[u8], key: &[u8; 32], iv: &[u8; 16]) -> String {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();

    return String::from_utf8(cipher.decrypt_vec(ciphertext).unwrap()).unwrap();
}

//TODO add more advanced AES encryption
//TODO: implement encryption for exe files, images, and other file types
fn main() {
    let keypair = AESKeypairImpl::new();
    // ^^ creates a new AES keypair implementation

    let key = keypair.generate_key();
    // ^^^ generates a 32 byte key
    let iv = keypair.generate_iv();
    // ^^^ generates a 16 byte iv

    let dir_path = Path::new("C:/Users/creid/RustroverProjects/file-encryption");

    let file_path = dir_path.join("message.txt");
    // ^^ creates a path to the message.txt file


    let message = std::fs::read_to_string(file_path).unwrap();
    // ^^ reads the contents of the message.txt file into a string


    let ciphertext = encrypt_message(&message, &key, &iv);
    // ^^ encrypts the message using the key and iv


    let encrypted_file_path = dir_path.join("encrypted_message.txt");
    // ^^ creates a path to the encrypted_message.txt file


    // if the path exists, remove the file
    if encrypted_file_path.exists() {
        std::fs::remove_file(&encrypted_file_path).unwrap();
    }

    std::fs::write(encrypted_file_path, &ciphertext).unwrap();
    // write the ciphered text to the encrypted file


    let decrypted_message = decrypt_message(&ciphertext, &key, &iv);
    println!("Decrypted message: {}", decrypted_message);
}
