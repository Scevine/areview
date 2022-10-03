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

