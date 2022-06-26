use uuid::Uuid;

#[derive(Debug)]
pub struct Expense {
    id: [u8; 16],
    name: String,
    tag: String,
    date_str: String,
    amount: i32,
}

impl PartialEq<Expense> for Expense {
    fn eq(&self, other: &Expense) -> bool {
        self.id == other.id
    }
}

impl Expense {
    pub fn new(id: [u8; 16], name: String, tag: String, date_str: &str, amount: i32) -> Expense {
        return Expense {
            id,
            name,
            tag,
            date_str: String::from(date_str),
            amount,
        };
    }

    pub fn empty() -> Expense {
        Expense {
            id: Uuid::new_v4().to_bytes_le(),
            name: String::new(),
            tag: String::new(),
            date_str: String::new(),
            amount: 0,
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

    pub fn date(&self) -> &str {
        return &self.date_str;
    }

    pub fn amount(&self) -> i32 {
        return self.amount;
    }

    pub fn set_id(&mut self, id: [u8; 16]) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = String::from(name);
    }

    pub fn set_tag(&mut self, tag: &str) {
        self.tag = String::from(tag);
    }

    pub fn set_date(&mut self, date_str: &str) {
        self.date_str = String::from(date_str);
    }

    pub fn set_amount(&mut self, amount: &str) {
        self.amount = amount.parse::<i32>().unwrap();
    }
}
