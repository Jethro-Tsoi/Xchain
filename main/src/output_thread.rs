// pub use output_thread;

use core::blockchain;
use std::{ 
    thread,
    time::Duration, 
    sync::{
        Mutex, 
        Arc
    },
};    

pub fn output_thread(bc: Arc<Mutex<blockchain::BlockChain>>) {
    thread::spawn(move || {
        loop {
            let bc = bc.lock().unwrap();
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
}