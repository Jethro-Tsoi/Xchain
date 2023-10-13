use core::{blockchain, account::Account};
use std::{ 
    sync::{Arc, Mutex}, 
    collections::HashMap,
};   
use cli::menu::Menu; 

fn main() {

    let bc = blockchain::BlockChain::new_blockchain();
    let bc = Arc::new(Mutex::new(bc));

    let mut account_map: HashMap<String, Account> = HashMap::new();

    let menu = Menu::new(bc, account_map);
    menu.execute();
    
    
}


