use rusqlite::{Connection, Result};
use crate::players::BankDatabase;

struct StockDatabase {
    connection: Connection,
}

fn sufficient_balance(player_id: u32) -> bool {
    true
}

impl StockDatabase {
    pub fn connection() -> Result<Self> {
        let connection = Connection::open("database.db")?;
        Ok(Self { connection})
    }

    pub fn current_price(&self, stock_id: u32) -> Result<u32> {
        Ok(1)
    }

    pub fn buy(&self, player_id: u32, stock_id: u32, quantity: u32, bank_db: &BankDatabase) ->Result<String> {
        if sufficient_balance(player_id){
            let amount = self.current_price(stock_id).unwrap() * quantity;
            let _ = BankDatabase::add_balance(bank_db, player_id, amount as i32);
            Ok(format!("{quantity} shares bought at total expense of {amount}"))
        } else {
            Ok("Not enough money bruh".to_string())
        }
    }
}


