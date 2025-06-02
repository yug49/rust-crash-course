#[derive(Debug)]
pub struct Account {
    pub address: String,
    pub ether: u32,
}

pub fn new(address: String) -> Account {
    Account { address, ether: 0 }
}
