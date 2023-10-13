use core::blockchain;
use std::{ 
    thread,
    time::Duration, 
    sync::{
        Mutex, 
        Arc
    },
};    

pub fn print(bc: Arc<Mutex<blockchain::BlockChain>>){
    thread::spawn(move ||{
        loop {
            let bc = bc.lock().unwrap();
            println!("++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++");
            for (block_no, b) in bc.blocks.iter().enumerate() {
                
                println!("This is the block {}", block_no);
                println!("{:#?}", b);
                println!("");

            }
            drop(bc);

            // todo : cal menu
            // return ;

            thread::sleep(Duration::from_secs(5));
        }
    });
}