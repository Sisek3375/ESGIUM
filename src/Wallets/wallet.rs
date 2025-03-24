use bitcoincrash_addr::{Address, HashType, Scheme};
use crypto::digest::Digest;
use crypto::ed25519;
use rand::RngCore;
use rand::rngs::OsRng;
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]

// Structure d'un wallet
pub struct Wallet {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
}
// Mélangeur de bytes
impl Wallet {
    fn new() -> Self {
        let mut key: [u8; 32] = [0; 32];
        OsRng.fill_bytes(&mut key);
        let (secrect_key, public_key) = ed25519::keypair(&key);

        let secret_key: Vec<u8> = secrect_key.to_vec();
        let public_key: Vec<u8> = public_key.to_vec();
    
        Wallet{
            secret_key,
            public_key,
        }
  }
  // Fonction pour générer des clés
  fn get_address(&self) -> String {
    let mut pub_hash: Vec<u8> = self.public_key.clone();
    hash_pub_key(&mut pub_hash);

    let address = Adress {
        body: pub_hash,
        scheme: Scheme::Base58, // Base58 supprime les caractères 0, O, 1 et I afin d'éviter de les confondre
        hash_type: HashType::Script,
        ..Default::default()
    };

    address.encode().unwrap()
  }

}

// Hashage de la clé publique
pub fn hash_pub_key(pub_key: &mut Vec<u8>) {
  let mut hasher1 : Sha256 = Sha256::new();
  hasher1.input(pub_key);
  hasher1.result(pub_key);
  let mut hasher2 : Ripemd160 = Ripemd160::new();
  hasher2.input(pub_key);
  pub_key.resize(20, 0);
  hasher2.result(pub_key);
}


pub struct Wallets {
    Wallets: HashMap<String, Wallet>,
}


