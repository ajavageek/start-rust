#[derive(Debug, PartialEq)]
pub struct Super {
    pub super_name: String,
    pub real_name: String,
    pub power: u16,
}

#[derive(Debug, PartialEq)]
pub struct Group {
    pub name: String,
    pub members: Vec<Super>,
}
