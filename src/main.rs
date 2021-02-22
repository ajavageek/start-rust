fn main() {
    println!("Hello, world!");
}

pub struct Super {
    pub super_name: String,
    pub real_name: String,
    pub power: u16,
}

pub struct Group {
    pub name: String,
    pub members: Vec<Super>,
}
