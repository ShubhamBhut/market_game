use rusqlite::{Connection, Result};


pub fn create_account(player_id: i32, username: String, password: String) -> Result<()>{
    let conn = Connection::open("database.db")?;
    let query = "INSERT INTO Players (player_id, username, password) VALUES (?1, ?2, ?3)";
    conn.execute(query, (player_id, username, password))?;
    Ok(())
}

pub fn check_balance(player_id: i32) -> Result<(f64)> {
    let conn = Connection::open("database.db")?;
    let query = "SELECT balance FROM Players WHERE player_id = (?1)";
    let mut stmt = conn.prepare(query)?;
    let mut rows = stmt.query((player_id,))?;
    if let Some(row) = rows.next()? {
        let balance = row.get::<usize, f64>(0)?;
        Ok(balance)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}

pub fn transfer(sender_id: i32, receiver_id: i32, amount: f32) ->Result<()> {
    let conn = Connection::open("database.db")?;
    let query1 = "UPDATE Players SET balance = balance - (?2) WHERE player_id = (?1)";
    conn.execute(query1, (sender_id, amount))?;
    let query2 = "UPDATE Players SET balance = balance + (?2) WHERE player_id = (?1)";
    conn.execute(query2, (receiver_id, amount))?;
    Ok(())
}

pub fn add_balance(receiver_id: i32, amount: f32) ->Result<()> {
    let conn = Connection::open("database.db")?;
    let query = "UPDATE Players SET balance = balance + (?2) WHERE player_id = (?1)";
    conn.execute(query, (receiver_id, amount))?;
    Ok(())
}

