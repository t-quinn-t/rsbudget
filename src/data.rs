use crate::errors::Error;

use std::fs;

use rusqlite::Connection;
use chrono::prelude::{Date, Local};

extern crate log;
extern crate pretty_env_logger;

pub struct DataStore {
    conn: Connection,
}

impl DataStore {

    pub fn new() -> Result<DataStore, Error> {

        Ok(DataStore {
            conn: DataStore::init_db()?
        })
    }

    pub fn add_expense(amount: i32, date: Date<Local>, name: Option<String>, tag: String) -> Result<(), Error> {

        Ok(())
    }

    fn init_db() -> Result<Connection, Error> {

        // Connect to db
        let mut path = dirs::config_dir().unwrap();
        path.push("rbudget");
        path.push("data");
        fs::create_dir_all(&path)?;
        path.push("em.db");
        let conn = Connection::open(&path)?;

        // Create tables if not exists
        let sql = "
            CREATE TABLE IF NOT EXISTS expenses (
                uuid BLOB NOT NULL PRIMARY KEY, 
                name VARCHAR(255) NOT NULL, 
                tag VARCHAR(255) NOT NULL,
                date INTEGER NOT NULL, 
                amount INTEGER NOT NULL   
            )
        ";
        conn.execute(sql, [])?;

        Ok(conn)

    }

}