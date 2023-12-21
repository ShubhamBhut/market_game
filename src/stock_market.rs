use rusqlite::{Connection, Result};
use crate::players::BankDatabase;

struct StockDatabase {
    connection: Connection,
}


impl StockDatabase {
    pub fn connection() -> Result<Self> {
        let connection = Connection::open("database.db")?;
        Ok(Self { connection})
    }

    pub fn current_price(&self, stock_id: u32) -> Result<u32> {
        Ok(1)
    }


    pub fn buy_stocks(&self, player_id: u32, stock_id: u32, quantity: u32, bank_db: &BankDatabase) ->Result<String> {
        let amount = self.current_price(stock_id).unwrap() * quantity;
        if bank_db.sufficient_balance(player_id, amount).unwrap(){
            let _ = BankDatabase::withdraw_balance(bank_db, player_id, amount );
            Ok(format!("{quantity} shares bought at total expense of {amount}"))
        } else {
            Ok("Not enough money bruh".to_string())
        }
    }

    pub fn sell_stocks(&self, player_id: u32, stock_id: u32, quantity: u32, bank_db: &BankDatabase) ->Result<String> {
        let amount = self.current_price(stock_id).unwrap() * quantity;
        let _ = BankDatabase::withdraw_balance(bank_db, player_id, amount );
        Ok(format!("{quantity} shares sold at total value of {amount}"))
    }
}


