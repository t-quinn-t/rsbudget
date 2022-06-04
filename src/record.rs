#[derive(Debug)]
pub struct Expense {
    id: [u8; 16],
    name: String,
    tag: String,
    date_timestamp: i64,
    amount: i32
}

impl PartialEq<Expense> for Expense {
    fn eq(&self, other: &Expense) -> bool {
        self.id == other.id 
    }
}

impl Expense {

    pub fn new(
        id: [u8; 16],
        name: String,
        tag: String,
        date: i64,
        amount: i32) -> Expense {
        return Expense {
            id, name, tag, date_timestamp: date, amount
        }
    }

    pub fn id(&self) -> [u8; 16] {
        return self.id;
    }

    pub fn name(&self) -> String {
        return self.name.clone();
    }

    pub fn tag(&self) -> String {
        return self.tag.clone();
    }

    pub fn date(&self) -> i64 {
        return self.date_timestamp;
    }

    pub fn amount(&self) -> i32 {
        return self.amount;
    }

}


