use crate::errors::Error;
use crate::record::Expense;

use std::fs;
use dotenv::dotenv;
use rusqlite::{Connection, params};

extern crate log;
extern crate pretty_env_logger;

trait DS {
    fn new() -> Result<Self, Error> where Self: Sized;
    fn mock() -> Result<Self, Error> where Self: Sized;
}

trait ExpenseDS {
    fn append_one(&self, epx:& Expense) -> Result<(), Error>;
    fn list_all(&self) -> Result<Vec<Expense>, Error>;
    fn remove_all(&self) -> Result<(), Error>;
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

    fn mock() -> Result<Self, Error> where Self:Sized {
        Ok(DataStore {
            conn: DataStore::init_testdb()?
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

    fn list_all(&self) -> Result<Vec<Expense>, Error> {
        
        let mut stmt = self.conn.prepare("
            SELECT * FROM expenses;
        ")?;
        let exp_iter = stmt.query_map([], |row| {
            Ok(Expense::new(
                    row.get("uuid")?,
                    row.get("name")?,
                    row.get("tag")?,
                    row.get("date")?,
                    row.get("amount")?
                    ))
        })?;   

        let mut exp_list = Vec::new();
        for expense in exp_iter {
            exp_list.push(expense?);
        }
        Ok(exp_list)
    }

    fn remove_all(&self) -> Result<(), Error> {
        
        Ok(())
    }
}
   
impl DataStore {

    fn init_testdb() -> Result<Connection, Error> {
        dotenv().ok();
       
        let testdb_url = dotenv::var("TEST_DATABASE_URL").unwrap();
        let conn = Connection::open(&testdb_url)?;
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

    use chrono::prelude::*;
    let ds: DataStore = DataStore::mock().unwrap();

    let test_uuid1 = uuid::Uuid::new_v4().to_bytes_le();
    let test_date1 = Local::today();
    let exp1 = Expense::new(test_uuid1, String::from("name1"), String::from("tag1"), test_date1.and_hms(0,0,0).timestamp(), 100);

    let test_uuid2 = uuid::Uuid::new_v4().to_bytes_le();
    let test_date2 = Local::today();
    let exp2 = Expense::new(test_uuid2, String::from("name2"), String::from("tag2"), test_date2.and_hms(0,0,0).timestamp(), 200);

    ds.append_one(&exp1).unwrap();
    ds.append_one(&exp2).unwrap();

    let expected_list1 = vec![exp1, exp2];
    let actual_list1 = ds.list_all().unwrap();
    assert_eq!(expected_list1[1], actual_list1[1]);
    assert_eq!(expected_list1[0], actual_list1[0]);

    // Clear up 
    let testdb_url = dotenv::var("TEST_DATABASE_URL").unwrap();
    let conn = Connection::open(&testdb_url).unwrap();
    conn.execute("
            DROP TABLE IF EXISTS expenses;
        ", []).unwrap();
}
