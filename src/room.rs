use crate::LabelColor;
use fnv::FnvHashMap;
use nannou::color::named::*;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Door {
    None,
    Closed,
}

impl Door {
    pub fn or(&self, other: &Door) -> Door {
        match (self, other) {
            (Door::None, Door::None) => Door::None,
            _ => Door::Closed,
        }
    }
}

pub type Vnum = u32;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    pub vnum: Vnum,
    pub string_vnum: String,
    pub exits: FnvHashMap<Direction, (Vnum, Door)>,
    pub sector: Sector,
    // pub safe: boolean,
    // pub cursed: boolean,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

    pub fn color(&self) -> LabelColor {
        match self {
            Sector::Inside => LabelColor::light(GAINSBORO),
            Sector::House => LabelColor::light(BISQUE),
            Sector::City => LabelColor::light(DARKGRAY),
            Sector::RogueGuild => LabelColor::dark(DARKSLATEGRAY),

            Sector::Field => LabelColor::light(LIGHTGREEN),
            Sector::Hills => LabelColor::light(MEDIUMSEAGREEN),
            Sector::Forest => LabelColor::dark(SEAGREEN),
            Sector::Mountain => LabelColor::dark(OLIVEDRAB),
            Sector::Desert => LabelColor::dark(OLIVE),

            Sector::WaterSwim => LabelColor::light(SKYBLUE),
            Sector::WaterNoswim => LabelColor::light(DEEPSKYBLUE),
            Sector::Underwater => LabelColor::dark(ROYALBLUE),
            Sector::OnBottom => LabelColor::dark(MEDIUMBLUE),

            Sector::Air => LabelColor::light(ALICEBLUE),
        }
    }
}
