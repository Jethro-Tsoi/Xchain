// pub type Address = [u8; 24];

#[derive(Debug)]
pub struct Account{
    address: String,
    balance: [f32; 256],
}

impl Account {
    pub fn new (address: String) -> Account{
        Account {
            address,
            balance: [0.0; 256], 
        }
    }

    pub fn get_balance(&self) -> [f32; 256] {
        self.balance
    }

    pub fn set_balance(&mut self, symbol_index: u8, balance: f32){
        self.balance[symbol_index as usize] = balance
    }
}