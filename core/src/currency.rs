use std::rc::{
    self,
    Rc,
};

#[derive(Debug)]
pub struct Currency {
    nonce: u8,
    symbol: String,
}

impl Currency{
    fn new(nonce: u8, symbol: String) -> Currency {
        Currency { nonce, symbol }
    }

    pub fn get_nonce(&self) -> u8 {
        self.nonce
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol
    }
}

pub struct CurrencyList {
    currency_list: Vec<Currency>,
}

impl CurrencyList {
    pub fn new_currency_list () -> CurrencyList {
        CurrencyList { currency_list: Self::initial_list() }
    }

    fn initial_list () -> Vec<Currency> {
        vec![
            Currency {
                nonce: 0,
                symbol: String::from("eHKD"),
            },
            Currency {
                nonce: 1,
                symbol: String::from("eGBP"),
            },
            Currency {
                nonce: 2,
                symbol: String::from("eJPY"),
            },
        ]
    }

    pub fn get_currency_list(&self) -> &Vec<Currency> {
        &self.currency_list
    }

    pub fn add_currency(&mut self, symbol: String) {
        let last_nonce = self.currency_list[ self.currency_list.len() -1 ].get_nonce();
        self.currency_list.push(
            Currency { nonce: last_nonce + 1, symbol }
        )
    }
}