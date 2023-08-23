use chrono::prelude::*;
use utils::coder;
use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockHeader {
    pub time: i64,
    pub tx_hash_root: String, // merkle tree
    pub pre_block_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub tx_data: String,
}

impl Block{
    fn set_hash(&mut self){
        let header = coder::serialize(&(self.header));

        self.hash = coder::get_hash(&header)
    }
    
    pub fn new(tx_data: &str, pre_block_hash: &str) -> Block {
        
        let transaction = coder::serialize(&tx_data);

        let tx_hash_root  = coder::get_hash(&transaction);

        let time = Utc::now().timestamp();

        let mut block = Block{
            header: BlockHeader { time, tx_hash_root, pre_block_hash: pre_block_hash.to_owned() },
            hash: "".to_owned(),
            tx_data: tx_data.to_owned(),
        };

        block.set_hash();

        block
    }
}