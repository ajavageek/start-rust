#[derive(Debug, PartialEq)]
pub struct Super<'a> {
    pub super_name: &'a str,
    pub real_name: &'a str,
    pub power: u16,
    pub sidekick: Box<Option<Super<'a>>>,
    pub alignment: Alignment,
}

#[derive(Debug, PartialEq)]
pub struct Group<'a> {
    pub name: &'a str,
    pub members: Vec<Super<'a>>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Alignment {
    Good, Evil,
}
