use crate::errors::Error;

use std::fs;

use uuid::*;
use rusqlite::{Connection, params};
use chrono::prelude::{Date, Local};

extern crate log;
extern crate pretty_env_logger;

trait DS {
    fn new() -> Result<Self, Error> where Self: Sized;
}

trait ExpenseDS {
    fn append_one(&self, amount: i32, date: Date<Local>, name: String, tag: String) -> Result<(), Error>;
}

pub struct DataStore {
    conn: Connection,
}

impl DS for DataStore {
    fn new() -> Result<Self, Error> where Self:Sized {

        Ok(DataStore {
            conn: DataStore::init_db()?
        })
    }
}



impl ExpenseDS for DataStore {

    fn append_one(&self, amount: i32, date: Date<Local>, name: String, tag: String) -> Result<(), Error> {

        let id = Uuid::new_v4().to_bytes_le();
        let date_str= date.to_string();
        let stmt = "
            INSERT INTO expenses (uuid, name, tag, date, amount)
        ";
        self.conn.execute(stmt, params![id, name, tag, date_str, amount])?;

        Ok(()) 
    }
}
   
impl DataStore {

       fn init_db() -> Result<Connection, Error> {

        // Connect to db
        let mut path = dirs::config_dir().unwrap();
        path.push("rbudget");
        path.push("data");
        fs::create_dir_all(&path)?;
        path.push("em.db");
        let conn = Connection::open(&path)?;

        // Create tables if not exists
        let stmt = "
            CREATE TABLE IF NOT EXISTS expenses (
                uuid BLOB NOT NULL PRIMARY KEY, 
                name VARCHAR(255) NOT NULL, 
                tag VARCHAR(255) NOT NULL,
                date VARCHAR(255) NOT NULL, 
                amount INTEGER NOT NULL   
            )
        ";
        conn.execute(stmt, [])?;

        Ok(conn)

    }

}