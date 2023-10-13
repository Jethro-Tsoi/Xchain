use core::account::Account;
use core::{blockchain, currency};
use std::collections::HashMap;
use std::{process, io};
use std::sync::{
    Mutex, 
    Arc
};    
use core::currency::CurrencyList;
use crate::get_input::GetInput;


pub fn add_transaction(bc: Arc<Mutex<blockchain::BlockChain>>, account_map: &mut HashMap<String, Account>){

    loop {
        println!("Please enter a transaction: (e.g. a -> b, 10 eHKD -> 1 eGBP)");
        println!("Type 'menu' back to menu");
        println!("Type 'exit' to end");

        println!("Enter transferer: (e.g. a)");
        let transferer_address = get_input_line();
        let transferer_account = match account_map.get(&transferer_address[..]) {
            Some(a) => a,
            None => {
                println!("Account not exist");
                continue;
            },
        }; 

        println!("Enter receiver: (e.g. b)");
        let receiver_address = get_input_line();
        let receiver_account = match account_map.get(&receiver_address[..]) {
            Some(a) => a,
            None => {
                println!("Account not exist");
                continue;
            },
        }; 

        let currency_list = CurrencyList::new_currency_list();
        let currency_list = currency_list.get_currency_list();
        for currency in currency_list {
            println!("{:#?}", currency)
        }

        println!("Enter amount of currency transfer: (e.g. 10)");
        let transfer_currency_amount: f32 = match get_input_line().parse() {
            Ok(unit) => unit,
            _ => {
                println!("Please enter a correct number");
                continue;
            },
        };

        println!("Enter symbol of currency transfer: (e.g. 0 for eHKD)");
        let mut transfer_currency_symbol: usize = match get_input_line().parse() {
            Ok(symbol) if ( symbol <= currency_list.len() ) => symbol,
            _ => {
                println!("Please enter a correct number");
                continue;
            },
        };
        println!("Enter amount of currency receive: (e.g. 1)");
        let mut receive_currency_amount: f32 = match get_input_line().parse() {
            Ok(unit) => unit,
            _ => {
                println!("Please enter a correct number");
                continue;
            },
        };

        println!("Enter symbol of currency receive: (e.g. 1 for eGBP)");
        let mut receive_currency_symbol: usize = match get_input_line().parse() {
            Ok(symbol) if ( symbol <= currency_list.len() ) => symbol,
            _ => {
                println!("Please enter a correct number");
                continue;
            },
        };    

        let transferer_balance = &mut transferer_account.get_balance()[transfer_currency_symbol];
        if transfer_currency_amount > *transferer_balance{
            println!("Not enough amount for transfer account");
            continue;
        }
        *transferer_balance -= transfer_currency_amount;
        receiver_account.get_balance()[receive_currency_symbol] += receive_currency_amount;

        let tx_data = format!("{} -> {}, {} {} -> {} {}", 
                                        transferer_address, 
                                        receiver_address,
                                        transfer_currency_amount,
                                        (*currency_list[transfer_currency_symbol].get_symbol()).to_string(),
                                        receive_currency_amount,
                                        (*currency_list[receive_currency_symbol].get_symbol()).to_string());
        {
            let mut bc = bc.lock().unwrap();
            bc.add_block(&tx_data);
        }

        
    }
}
