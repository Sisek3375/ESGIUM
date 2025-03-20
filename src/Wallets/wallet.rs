use crypto::ed25519;
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize}

pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}

impl Wallet {
    fn new() -> Self {
        let mut key: [u8; 32] = [0; 32];
        OsRng.fill_bytes(dest: &mut key);
        let (secrect_key: [u8;64], public_key: [u8,32]) = ed25519::keypair(seed: &key);
        let secret_key: Vec<u8> = secrect_key.to_vec();
        let public_key: Vec<u8> = public_key.to_vec();
    
    Wallet{
        secret_key,
        public_key,
    }
  }

  fn get_address(&self) -> String {
    let mut pub_hash : Vec<u8> = self.public_key.clone();
    hash_pub_key(&mut pub_hash)
  }











}