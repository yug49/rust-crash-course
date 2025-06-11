pub fn print() {
    println!("a");
}

// Public struct
pub struct S {
    pub name: String,
    // Private field
    id: u32,
}

pub fn build(name: String) -> S {
    S { name, id: 1 }
}
