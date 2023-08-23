use core::blockchain;
use std::{ 
    io::{self},
    thread,
    time::Duration, sync::{Mutex, Arc},
};    


fn main() {
    let bc = blockchain::BlockChain::new_blockchain();
    let bc = Arc::new(Mutex::new(bc));
    
    let bc_for_output = Arc::clone(&bc);
    let t1 = thread::spawn(move || {
        loop {
            let bc = bc_for_output.lock().unwrap();
            println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
            for (block_no, b) in bc.blocks.iter().enumerate() {
                
                println!("This is the block {}", block_no);
                println!("{:#?}", b);
                println!("");
            }
            drop(bc);
            thread::sleep(Duration::from_secs(5));
        }
    });

    loop {
        println!("Please enter a transaction: (e.g. a -> b, 10 eHKD -> 1 eGBP)");
        println!("Type 'exit' to end");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input = input.trim().to_string();

        {
            let mut bc = bc.lock().unwrap();
            bc.add_block(&input);
        }

        if input == "exit" {
            break;
        }
    }
}
