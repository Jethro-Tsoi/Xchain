use core::blockchain;


fn main() {
    
    let mut bc = blockchain::BlockChain::new_blockchain();

    for b in bc.blocks {
        println!("+++++++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }
}
