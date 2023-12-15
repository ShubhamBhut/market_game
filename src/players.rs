use rusqlite::{Connection, Result};

struct BankDatabase {
    connection: Connection,
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

    pub fn check_balance(&self, player_id: u32) -> Result<i32> {
        let query = "SELECT balance FROM Players WHERE player_id = (?1)";
        let mut stmt = self.connection.prepare(query)?;
        let mut rows = stmt.query((player_id,))?;
        rows.next()?.map(|row| row.get(0))
            .ok_or(rusqlite::Error::QueryReturnedNoRows)?
    }

    fn add_balance(&self, receiver_id: u32, amount: i32) -> Result<()> {
        let query = "UPDATE Players SET balance = balance + (?2) WHERE player_id = (?1)";
        self.connection.execute(query, (receiver_id, amount))?;
        let balance = self.check_balance(receiver_id).unwrap();
        Ok(())
    }

    fn transfer(&self, sender_id: u32, receiver_id: u32, amount: u32) ->Result<()> {
        self.add_balance(sender_id, -1*(amount as i32));
        self.add_balance(receiver_id, amount as i32);
        Ok(())
    }

    pub fn donate(&self, sender_id: u32, receiver_id: u32, amount: u32) -> Result<String> {
        self.transfer(sender_id, receiver_id, amount)?;
        Ok(format!("{amount} has been donated to {receiver_id} from {sender_id}"))
    }
}
