use rusqlite::{Connection, Result};
use std::error::Error;

pub struct BankDatabase {
    pub connection: Connection,
}

#[derive(Debug)]
pub struct BankDatabaseError {
    pub message: String,
}

impl Error for BankDatabaseError {}

impl std::fmt::Display for BankDatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Bank database error: {}", self.message)
    }
}

impl BankDatabase {
    pub fn connection() -> Result<Self> {
        let connection = Connection::open("database.db")?;
        Ok(Self { connection })
    }

    fn latest_player_id(&self) -> Result<u32> {
        let query = "SELECT player_id FROM Players ORDER BY player_id DESC LIMIT 1";
        let mut stmt = self.connection.prepare(query)?;
        let mut rows = stmt.query(())?;
        rows.next()?.map(|row| row.get(0))
            .ok_or(rusqlite::Error::QueryReturnedNoRows)?
    }

    pub fn create_account(&self, username:&str , password: &str) -> Result<String> {
        let player_id = self.latest_player_id().unwrap() + 1;
        let query = "INSERT INTO Players (username, password) VALUES (?1, ?2)";
        self.connection.execute(query, (username, password))?;
        Ok(format!("New account created with player_id: {} and username: {}", player_id, username))
    }

    pub fn check_balance(&self, player_id: u32) -> Result<u32> {
        let query = "SELECT balance FROM Players WHERE player_id = (?1)";
        let mut stmt = self.connection.prepare(query)?;
        let mut rows = stmt.query((player_id,))?;
        rows.next()?.map(|row| row.get(0))
            .ok_or(rusqlite::Error::QueryReturnedNoRows)?
    }

    pub fn add_balance(&self, player_id: u32, amount: u32) -> Result<()> {
        let query = "UPDATE Players SET balance = balance + (?2) WHERE player_id = (?1)";
        self.connection.execute(query, (player_id, amount))?;
        Ok(())
    }

    pub fn sufficient_balance(&self, player_id: u32, required_balance: u32) ->Result<bool, BankDatabaseError> {
        let player_balance = self.check_balance(player_id);

        if player_balance.unwrap() >= required_balance {
           Ok(true)
        } else {
            Err(BankDatabaseError { message: "Insufficient Funds".to_string() })
        }
    }

    pub fn withdraw_balance(&self, player_id: u32, amount: u32) -> Result<()> {
        if self.sufficient_balance(player_id, amount).unwrap(){
            let query = "UPDATE Players SET balance = balance - (?2) WHERE player_id = (?1)";
            self.connection.execute(query, (player_id, amount))?;
        } 
       Ok(())
    }

    pub fn transfer(&self, sender_id: u32, receiver_id: u32, amount: u32) ->Result<()> {
        let _= self.withdraw_balance(sender_id, amount);
        let _= self.add_balance(receiver_id, amount);
        Ok(())
    }

    pub fn donate(&self, sender_id: u32, receiver_id: u32, amount: u32) -> Result<String> {
        self.transfer(sender_id, receiver_id, amount)?;
        Ok(format!("{amount} has been donated to {receiver_id} from {sender_id}"))
    }

    pub fn charge_commission(&self, player_id: u32, amount: u32, commission_percent: u32) ->Result<()> {
        let commission_amount = (commission_percent * amount) / 100;
        let query = "UPDATE Plyaers SET balance = balance - (?2) WHERE player_id = (?1)";
        self.connection.execute(query, (player_id, commission_amount))?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test() -> BankDatabase {
        let connection = Connection::open("test.sqli").expect("Failed to create database connection");
        BankDatabase { connection }
    }

    fn del_user(player_id: u32) -> Result<()>{
        let bank_db = setup_test();
        let query = "DELETE FROM Players WHERE player_id = (?1)";
        bank_db.connection.execute(query, (player_id,))?;
        Ok(())
    }

    #[test]
    fn test_create_account() {
        let bank_db = setup_test();
        let _ = bank_db.create_account("username3", "password3");
        assert!(bank_db.latest_player_id().unwrap() == 3);
        let _ = del_user(3);
    }

    #[test]
    fn test_check_balance() {
        let bank_db = setup_test();
        let result = bank_db.check_balance(1); // Assuming player_id 1 exists in your test database
        assert!(result.unwrap() == 1000);
    }

    #[test]
    fn test_donate() {
        let bank_db = setup_test();
        let _ = bank_db.create_account("username3", "password3");
        let _ = bank_db.donate(2, 3, 10);
        assert!(bank_db.check_balance(3).unwrap() == 10);
        assert!(bank_db.check_balance(2).unwrap() == 20);
        let _ = bank_db.add_balance(2, 10);
        let _ = del_user(3);
    }
}

