#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;

pub mod client;
pub mod schema;
pub mod models;

use models::*;
use std::error::Error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub struct BotsDBClient {
    conn: PgConnection,
}

impl BotsDBClient {
    pub fn new() -> Result<BotsDBClient, Box<dyn Error>> {
        let conn = BotsDBClient::establish_conn()?;

        Ok(BotsDBClient {
            conn: conn,
        })
    }

    pub fn init(&mut self) {
        use schema::bots::server::dsl::*;

        let results = server.load::<Server>(&self.conn).expect("error loading servers");

        for result in results {
            println!("{}", result.name);
        }
    }

    fn establish_conn() -> Result<PgConnection, Box<dyn Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let conn = PgConnection::establish(&database_url)?;

        return Ok(conn);
    }
}
