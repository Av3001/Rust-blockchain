extern crate bincode;
use bincode::{serialize, deserialize};
use crate::{ProofOfWork, Transaction};
use serde::{Deserialize, Serialize};

pub struct Block {
    timestamp: i64,
    pre_block_hash: String,
    hash: String,
    transactions: Vec<Transaction>,
    nonce: i64,
    height: usize,
}


impl Block{
    pub fn new_block(pre_block_hash: String, transactions: &[Transaction], height: usize) -> 
    Block {
        let mut block = Block {
            timestamp: crate::current_timestamp(),
            pre_block_hash,
            hash: String::new(),
            transactions: transactions.to_vec(),
            nonce: 0,
            height,
         }; 
        let pow = ProofOfWork::new_proof_of_work(block.clone());
        let (nonce, hash) = pow.run();
        block.nonce = nonce;
        block.hash = hash;
        return block; 
        }
    pub fn deserialize(bytes: &[u8]) -> Block {
        bincode::deserialize(bytes).unwrap()
        }
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap().to_vec()
        }
    pub fn get_transactions(&self) -> &[Transaction] {
        self.transactions.as_slice()
        } 
    pub fn get_pre_block_hash(&self) -> String {
        self.pre_block_hash.clone()
        }    
}        