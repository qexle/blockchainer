extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transcation{
    sender: String,
    receiver: String,
    amount: f32,
}

//Header struct for our block.
#[derive(Serialize, Debug)]
pub struct Blockheader{
    timestamp: i64,
    nonce: u32,
    pre_hash: String.
    merkle: String,
    difficutly: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,//amount of transactions in the block.
    transactions: Vec<Transcation>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transcation>,
    difficutly: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficutly: u32) -> Chain {
        //We'll create a chain by binding it to a mutable value chain.
        let mut chain = Chain {
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficutly,
            miner_addr,
            reward: 100.0,//when our chain starts we have initially 100 coins.
            //and then we can decide that how we want to distribute the coins.
        };
        chain.generate_new_block();
        chain//return our chain from this function.
    }
    pub fn generate_new_block(&mut self, sender: String, reciever: String, amount: f32) -> bool {
        //generate a new block with the given values.
        self.curr_trans.push(Transcation{
            sender,
            reciever, 
            amount,
        });
        true
    }
    pub fn last_hash(&self) -> String{
        //self if the sturct we are implementing means Chain.
        let block = match self.chain.last(){
            //we'll bind the last block of the chain to block.
            //if we get a block we output it otherwise we output 
            //a vector with 48 zeros.
            None => return String::from_utf8(vec![48; 64]).unwrap()
            //this is accounting for our genesis block.
            //when we create our chain the last hash won't exist.
            //so we need to fill it out with someting so we reuturn zeros.

        };
        Chain::hash(&block.header)
        pub fn update_difficulty(&mut self, difficutly: u32) -> bool {{
            self.difficutly = difficutly;
            true
        }
        pub fn update_reward(&mut self, reward: f32) -> bool {
            self.reward = reward;
            true
        }
        pub fn generate_new_block(&mut self) -> bool {
            let header = Blockheader{
                timestamp: time::now().to_timespec().sec,
                nonce: 0,
                merkle: String::new(),
                pre_hash: self.last_hash(),
                difficutly: self.difficutly,
            };

            let reward_trans = Transcation{
                sender: String::from("Root"),
                reciever: self.miner_addr.clone(),
                amount: self.reward
            };

            let mut block = Block {
                header,
                count: 0,
                transactions: vec![]
            };
            block.transactions.push(reward_trans);
            block.transactions.append(&mut self.curr_trans);
            block.count = block.transactions.len() as u32;
            block.header.merkle = Chain::get_merkle(block.transactions.clone());
            //function will calculate merkle hash accordingly.
        Chain::proof_of_work(&mut block.header);
        //function will calculate the proof of work.
        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            //This will iterate untill we've
            //a singly length inside of our merkle vector.
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        //when we finally have a single lenght we pop it off 
        //and return it.
        merkle.pop().unwrap()
    }
    //final hash that comes from combining all the other hashes
    //inside of the vector will be the hash that will be our
    //quote unquote merkle hash.
    pub fn proof_of_work(header: &mut Blockheader) {
        loop {//as difficulty goes up we need to loop more and mpre
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }
    //This function will loop around untill we have 
    //the proper nonce it gives us a hash with a certain
    //number of leading zeros.
    //difficulty: 2
    //then
    //hash: 00832478347583492504950

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.input(input.as_bytes());
        let res = hasher.result();
        let vec_res = res.to_vec();

        Chain::hex_to_string(vec_res.as_slice())
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }

}
    }