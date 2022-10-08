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

impl Direction {
    pub fn opposite(&self) -> Direction {
        use Direction::*;
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
            Up => Down,
            Down => Up,
        }
    }
}

pub type Vnum = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    pub vnum: Vnum,
    pub exits: FnvHashMap<Direction, Vnum>,
    pub sector: Sector,
    // pub safe: boolean,
    // pub cursed: boolean,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Sector {
    Inside,
    City,
    Field,
    Forest,
    Hills,
    Mountain,
    WaterSwim,
    WaterNoswim,
    House,
    Air,
    Desert,
    Underwater,
    OnBottom,
    RogueGuild,
}

impl Sector {
    pub fn from_str(s: &str) -> Sector {
        use Sector::*;
        match s {
            "0" => Inside,
            "1" => City,
            "2" => Field,
            "3" => Forest,
            "4" => Hills,
            "5" => Mountain,
            "6" => WaterSwim,
            "7" => WaterNoswim,
            "8" => House,
            "9" => Air,
            "10" => Desert,
            "11" => Underwater,
            "12" => OnBottom,
            "13" => RogueGuild,
            _ => Inside,
        }
    }
}
