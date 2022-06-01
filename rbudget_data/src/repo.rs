use chrono::{Date, Local};


pub struct Expense {
    amount: i32,
    date: Date<Local>,
    name: String, 
    tag: String
}