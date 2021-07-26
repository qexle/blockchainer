//Blockchain is like a database and we can store any kind of data on it.
//Transcation records are most often stored.
//Every blockchain has previos hash, timestamp, TX_ID(Transcation root) which are 
//filled with bunch of transcation and hashed and a Nonce.
//Transcation root also called a merkle root.

//When a new transcation comes in it gets stored on a block and then.
//and the block enters the chain after somebody starts mining the bock.
//when people start mining the block and the block is shown up on the blockchain.
// How mining works is so we get the previous hash and we start throwing random bits
//into an algorithm based on the nounce we have untill we get a pattern.
//If the pattern matches and it is corresponds to the nounce so we mine the block and 
//the block gets produced on the blockchain.
//difficutly corresponds to the number of leading zeros in the hash.
//most block chains are distributed which means that we can have multiple
//different blockchains that have the same data on multiple different computers(nodes).
//if the block chain changes on a single computer then other chains on every computer
//will tell that it was changed and they will update their data.
// time lib so we'll be able to get time stamps
// serde allows us to serialize and deserialize data.
// so it allows to basically put our blockchain into a JSON format.
// sha2 allows us to create sha2 hashes for our blockchain.
#[macro_use]//This will allow us to derive all the serializes 
//from blockchain.rs on our main.rs

extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod blockchain;//import blockchain.rs header file

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("Enter a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);
    print!("Difficulty: ");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("Enter an integer");
    println!("Generating genesis block !...");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) New Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 =>//cases 0, exit
            {
                println!("exiting!");
                process::exit(0);
            },
            1 => {//cases 1, new transaction
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                print!("Enter sender address:");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);
                print!("Enter receiver address: ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);
                print!("Enter amount: ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(sender.trim().to_string(), 
                                        receiver.trim().to_string(), 
                                        amount.trim().parse().unwrap());

                match res {//What new_transaction function returns.
                    true => println!("Transaction Sucessful"),
                    false => println!("Transaction Failed"),
                }
            },
            2 =>//cases 2, mine block
            {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            },
            3 =>//cases 3, change difficulty
            {
                let mut new_diff = String::new();
                print!("Enter new difficulty: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed Update Difficulty"),
                }
            },
            4 =>{//cases 4, change reward
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            _ => println!("Invalid option please retry"),
            //input other then 0, 1, 2, 3, 4
        }
    }
}