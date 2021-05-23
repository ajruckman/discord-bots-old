extern crate discord_bots_db;

use discord_bots_db::BotsDBClient;

fn main() {
    println!("Hello, world!");

    let mut db = BotsDBClient::new().unwrap();
}
