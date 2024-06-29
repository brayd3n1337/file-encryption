use rand::RngCore;
use rand::rngs::OsRng;

pub struct AESKeypairImpl;

pub trait AESKeypair {
    fn generate_key(&self) -> [u8; 32];

    fn generate_iv(&self) -> [u8; 16];

    fn new() -> AESKeypairImpl {
        AESKeypairImpl
    }
}

impl AESKeypair for AESKeypairImpl {
    fn generate_key(&self) -> [u8; 32] {
        let mut key = [0u8; 32];
        // ^^ creates a new 32 byte array


        OsRng::default().fill_bytes(&mut key);
        // ^^ fills the key with random bytes

        return key;
    }

    fn generate_iv(&self) -> [u8; 16] {
        let mut iv = [0u8; 16];
        // ^^ creates a new 16 byte array
        OsRng::default().fill_bytes(&mut iv);
        // ^^ fills the iv with random bytes

        return iv;
    }
}

