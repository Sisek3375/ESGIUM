use crate::errors::Result;
use std::time::SystemTime;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use log::info;
const TARGET_HEXT:usize = 4;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]

// Structure Block
pub struct Block {
    timestamp: u128,
    transactions: String,
    prev_block_hash: String,
    hash: String,
    height: usize,
    nonce: i32,
}

// Structure de Blockchain
pub struct Blockchain {
    blocks: Vec<Block>,
}

// Implementation de Block
impl Block {
    pub(crate) fn get_prev_hash(&self) -> String{
        self.prev_block_hash.clone()
    }
    }
    // PoW
    fn run_proof_of_work(&mut self) -> Result<()> {
        info!("Mining the block...");
        while !self.validate()? {
            self.nonce += 1; //tant que le hash est non valide, incrémenter de 1 nonce
        }
        let data:Vec<u8> = self.prepare_hash_data()?;
        let mut hasher:Sha256 = Sha256::new();
        hasher.input(&data);
        self.hash = hasher.result_str();
        Ok(())
    }
    // Preparation du hash
    fn prepare_hash_data(&self) -> Result<Vec<u8>> {
    let content = (
            self.prev_block_hash.clone(),
            self.transactions.clone(),
            self.timestamp,
            TARGET_HEXT,
            self.nonce,
            );
        let bytes:Vec<u8> = bincode::serialize(&content)?;
        Ok(bytes)
    }
    // Validation du hash
    fn validate(&self) -> Result<bool> {
        let data: Vec<u8> = self.prepare_hash_data()?;     
        let mut hasher = Sha256::new();                    
        hasher.input(&data);                               
    
        let hash_str = hasher.result_str();   
        let target_prefix = "0".repeat(TARGET_HEXT);
    
        println!("generated hash : {}", hash_str); 
        println!("target: {}", target_prefix);
    
        Ok(&hash_str[..TARGET_HEXT] == target_prefix)
    }

    // Implementation de blockchain
    impl Blockchain {
        pub fn new() -> Blockchain {
            Blockchain {
                blocks: vec![Block::new_genesis_block()]
            }
        }
    pub fn add_block(&mut self, data: String) -> Result<()>{
       let prev :&Block = self.blocks.last().unwrap();
       let new_block = Block::new_block(data, prev.get_hash(), prev.height + 1)?;
       self.blocks.push(new_block);
       Ok(())
    }
}