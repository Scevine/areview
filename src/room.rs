use fnv::FnvHashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

pub type Vnum = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    pub vnum: Vnum,
    pub exits: FnvHashMap<Direction, Vnum>,
}

#[derive(Debug, Copy, Clone, Eq, Hash)]
pub struct Connection(pub Vnum, pub Vnum);

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}
