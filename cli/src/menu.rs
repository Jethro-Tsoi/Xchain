use core::{blockchain::BlockChain, account::Account};
use std::{sync::{Arc, Mutex}, collections::HashMap};

use crate::{get_input::{self, GetInput}, printer_blockchain, add_transaction, add_account};

pub struct Menu {
    bc: Arc<Mutex<BlockChain>>, 
    account_map: HashMap<String, Account>,
    // get_input: Box<GetInput>,
}

impl Menu {
    pub fn new (bc: Arc<Mutex<BlockChain>>, account_map: HashMap<String, Account>) -> Menu {
        Menu { bc, account_map }
    }

    pub fn execute () -> bool {
        loop {
            println!();
            println!();
            println!();
            println!("Enter your action with following code");
            println!("0 : view the full blockchain");
            println!("1 : add a transaction");
            println!("2 : add a account");
            println!("3 : view a account");
            println!("4 : view all account");
            println!("4 : view currency support");

            let input: u8 = match GetInput.parse() {
                Ok(value) => value,
                Err(_) => continue,
            };

            let bc = Arc::clone(&bc);
            match input {
                0 => {
                    printer_blockchain::print(bc);
                } ,
                1 => {
                    add_transaction::add_transaction(bc, &mut account_map);
                },
                2 => {
                    add_account::add_account(&mut account_map);
                }
                _ => {
                    println!("Please enter a correct number");
                },
            };
        }
    }
}