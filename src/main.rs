mod database;
mod players;

fn main() {
    println!("Hello, world!");
    database::db_creation::create_db();
    println!("Database generated");

    players:: create_account(2, "second_player".to_string(), "password".to_string());
    println!("account created");

    let balance = players::check_balance(1);
    println!("balance is: {:?}", balance.unwrap());

    players::add_balance(1, 100.0);
    players::transfer(1, 2, 10.0);
    let balance = players::check_balance(2);
    println!("balance is {:?}", balance.unwrap());
}
