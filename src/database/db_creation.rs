use rusqlite::{Connection, Result};

pub fn create_db() -> Result<()> {
    let conn = Connection::open("database.db")?;

    // Players table
    let create_players_table = "CREATE TABLE Players (
        player_id INTEGER PRIMARY KEY,
        username TEXT UNIQUE NOT NULL,
        password TEXT NOT NULL,
        balance INTEGER NOT NULL DEFAULT 0.0,
        crypto_wallet_address TEXT DEFAULT NILL
    )";
    let mut stmt = conn.prepare(create_players_table)?;
    stmt.execute([])?;

    // Stocks table
    let create_stocks_table = "CREATE TABLE Stocks (
        stock_id INTEGER PRIMARY KEY,
        symbol TEXT UNIQUE NOT NULL,
        name TEXT NOT NULL,
        current_price INTEGER NOT NULL,
        quantity INTEGER NOT NULL DEFAULT 0,
        brokerage_fee INTEGER NOT NULL DEFAULT 0.0
    )";
    let mut stmt = conn.prepare(create_stocks_table)?;
    stmt.execute([])?;

    // Player_Stocks table
    let create_player_stocks_table = "CREATE TABLE Player_Stocks (
        player_id INTEGER NOT NULL REFERENCES Players(player_id),
        stock_id INTEGER NOT NULL REFERENCES Stocks(stock_id),
        quantity INTEGER NOT NULL DEFAULT 0,
        PRIMARY KEY (player_id, stock_id)
    )";
    let mut stmt = conn.prepare(create_player_stocks_table)?;
    stmt.execute([])?;

    // Real_Estate table
    let create_real_estate_table = "CREATE TABLE Real_Estate (
        property_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        price INTEGER NOT NULL,
        rent_amount INTEGER NOT NULL,
        owner_id INTEGER NOT NULL REFERENCES Players(player_id)
    )";
    let mut stmt = conn.prepare(create_real_estate_table)?;
    stmt.execute([])?;

    // Collectibles table
    let create_collectibles_table = "CREATE TABLE Collectibles (
        collectible_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        value REAL NOT NULL,
        owner_id INTEGER NOT NULL REFERENCES Players(player_id)
    )";
    let mut stmt = conn.prepare(create_collectibles_table)?;
    stmt.execute([])?;

    // Buffs table
    let create_buffs_table = "CREATE TABLE Buffs (
        buff_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        description TEXT,
        duration INTEGER NOT NULL,
        active BOOLEAN NOT NULL DEFAULT false,
        player_id INTEGER NOT NULL REFERENCES Players(player_id)
    )";
    let mut stmt = conn.prepare(create_buffs_table)?;
    stmt.execute([])?;

    // Transactions table
    let create_transactions_table = "CREATE TABLE Transactions (
        transaction_id INTEGER PRIMARY KEY,
        type TEXT NOT NULL,
        amount INTEGER NOT NULL,
        timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
        player_id INTEGER NOT NULL REFERENCES Players(player_id),
        other_player_id INTEGER REFERENCES Players(player_id),
        stock_id INTEGER REFERENCES Stocks(stock_id),
        property_id INTEGER REFERENCES Real_Estate(property_id),
        collectible_id INTEGER REFERENCES Collectibles(collectible_id)
    )";
    let mut stmt = conn.prepare(create_transactions_table)?;
    stmt.execute([])?;

    // Enterprise table
    let create_enterprise_table = "CREATE TABLE Enterprise (
        enterprise_id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        type TEXT NOT NULL,
        description TEXT,
        owner_id INTEGER NOT NULL REFERENCES Players(player_id),
        shareholders TEXT
    )";
    let mut stmt = conn.prepare(create_enterprise_table)?;
    stmt.execute([])?;


    Ok(())
}

