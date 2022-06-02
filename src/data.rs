use crate::errors::Error;
use crate::record::Expense;

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
    fn append_one(&self, epx:& Expense) -> Result<(), Error>;
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

    fn append_one(&self, exp:& Expense) -> Result<(), Error> {

        let stmt = "
            INSERT INTO expenses (uuid, name, tag, date, amount)
            VALUES (?1, ?2, ?3, ?4, ?5)
        ";
        self.conn.execute(stmt, params![
                          exp.id(),
                          exp.name(),
                          exp.tag(),
                          exp.date(),
                          exp.amount()
        ])?;

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
                date INTEGER NOT NULL, 
                amount INTEGER NOT NULL   
            )
        ";
        conn.execute(stmt, [])?;
        Ok(conn)
    }
}

// ++++++++++++++++++++++++ Unit Test ++++++++++++++++++++++++ //
#[test]
fn test_crud() {
    let ds: DataStore = DataStore::new().unwrap();

    let test_uuid1 = uuid::Uuid::new_v4().to_bytes_le();
    let test_date1 = Local::today();
    let exp1 = Expense::new(test_uuid1, String::from("name1"), String::from("tag1"), test_date1.and_hms(0,0,0).timestamp(), 100);

    let test_uuid2 = uuid::Uuid::new_v4().to_bytes_le();
    let test_date2 = Local::today();
    let exp2 = Expense::new(test_uuid2, String::from("name2"), String::from("tag2"), test_date2.and_hms(0,0,0).timestamp(), 200);

    ds.append_one(&exp1).unwrap();
    ds.append_one(&exp2).unwrap();
}


