use rusqlite::{Connection, Result};
use crate::players::BankDatabase;

struct StockDatabase {
    connection: Connection,
}

fn sufficient_balance(player_id: u32) -> bool {
    true
}

fn setup_test() -> BankDatabase {
        let connection = Connection::open("test.sqli").expect("Failed to create database connection");
        BankDatabase { connection }
    }

bank_db = Bankdatabase { Connection::open("database.db").expect("Failed to create database connection")};

impl StockDatabase {
    pub fn connection() -> Result<Self> {
        let connection = Connection::open("database.db")?;
        Ok(Self { connection })
    }

    pub fn buy(player_id: u32, stock_id: u32, quantity: u32) ->Result<()> {
        if sufficient_balance(player_id){
            let amount = current_price(stock_id) * quantity;
            BankDatabase::add_balance(bank_db, player_id, amount)
        } else {
            Ok("Not enough money bruh")
        }
    }
}


