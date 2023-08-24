use core::{
    blockchain, 
    account::{
        self, 
        Account
    }
};
use main::{
    output_thread,
    input_thread,
};
use std::{ 
    io,
    thread,
    time::Duration, 
    sync::{
        Mutex, 
        Arc
    }, 
    collections::HashMap,
};    

fn main() {

    let bc = blockchain::BlockChain::new_blockchain();
    let bc = Arc::new(Mutex::new(bc));
    
    let bc_for_output = Arc::clone(&bc);
    output_thread::output_thread(bc_for_output);

    input_thread::input(bc);
}


