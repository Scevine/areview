use fnv::FnvHashMap;
use once_cell::sync::Lazy;
use regex::{Captures, Match, Regex};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use std::str::FromStr;

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

type HashMap<T, V> = FnvHashMap<T, V>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Room {
    pub name: String,
    pub vnum: Vnum,
    pub exits: HashMap<Direction, Vnum>,
}

pub fn parse_rooms(text: &str) -> Result<Vec<Vec<Rc<Room>>>, Box<dyn Error>> {
    let room_section_regex = Regex::new(r"(?ims)^#ROOMS\s*$(.*?)^#0\s*$").unwrap();

    // TODO: write a PR to regex to let String be indexed by match
    let room_section = room_section_regex.captures(&text).ok_or(NoRoomsSection)?;

    let section_match = room_section.get(1).unwrap();
    let section_text = &text[section_match.start()..section_match.end()];

    let room_split_regex = Regex::new(r"(?m)^#(\d+)").unwrap();
    let room_matches: Vec<Match> = room_split_regex.find_iter(section_text).collect();
    let mut rooms = vec![];
    if room_matches.is_empty() {
        return Err(Box::new(NoRooms));
    }

    for matches in room_matches.windows(2) {
        let m = matches.get(0).unwrap();
        let next = matches.get(1).unwrap();

        match parse_room(section_text, m.start(), m.end(), next.start()) {
            Ok(room) => rooms.push(room),
            Err(e) => eprintln!("{e}"),
        }
    }

    let last_match = room_matches.iter().last().unwrap();
    match parse_room(
        section_text,
        last_match.start(),
        last_match.end(),
        section_text.len(),
    ) {
        Ok(room) => rooms.push(room),
        Err(e) => eprintln!("{e}"),
    }

    Ok(sort_rooms(rooms))
}

#[derive(Debug)]
struct NoRoomsSection;

impl Display for NoRoomsSection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No #ROOMS section detected in file")
    }
}

impl Error for NoRoomsSection {}

#[derive(Debug)]
struct NoRooms;

impl Display for NoRooms {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No rooms in #ROOMS section")
    }
}

impl Error for NoRooms {}

static ROOM_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?mx)\A\s*
        (?P<name>[^~]*)~
        [^~]*~\s*
        \d+\s+(?P<flags>[\d|]+)\s+(?P<sector>\d+)\s*",
    )
    .unwrap()
});

fn parse_room<'a>(
    text: &'a str,
    vnum_start: usize,
    vnum_end: usize,
    text_end: usize,
) -> Result<Room, Box<dyn Error + 'a>> {
    let vnum_text = &text[vnum_start + 1..vnum_end];
    let vnum = u32::from_str(vnum_text)?;
    let room_body = &text[vnum_end..text_end];
    let captures = ROOM_REGEX
        .captures(room_body)
        .ok_or(InvalidRoomBody { body: room_body })?;

    let name_match = captures.name("name").unwrap();
    let name = room_body[name_match.start()..name_match.end()].to_string();

    let captures_match = captures.get(0).unwrap();
    let exits = parse_doors(vnum, &room_body[captures_match.end()..]);

    Ok(Room { name, vnum, exits })
}

#[derive(Debug)]
struct InvalidRoomBody<'a> {
    body: &'a str,
}

impl<'a> Display for InvalidRoomBody<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid room body:\n{}", self.body)
    }
}

impl<'a> Error for InvalidRoomBody<'a> {}

static DOOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?imsx)
        ^D(?P<direction>\d+)
        [^~]*~
        [^~]*~\s*
        (?P<locks>\d+)\s+\d+\s+(?P<destination>\d+)",
    )
    .unwrap()
});

fn parse_doors(room_vnum: Vnum, text: &str) -> HashMap<Direction, Vnum> {
    let mut exits = HashMap::default();

    for captures in DOOR_REGEX.captures_iter(text) {
        match parse_door(text, captures) {
            Ok((direction, destination)) => {
                if exits.contains_key(&direction) {
                    eprintln!(
                        "Duplicate exit direction in room {}: {:?}",
                        room_vnum, direction
                    );
                } else {
                    exits.insert(direction, destination);
                }
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    exits
}

fn parse_door<'a>(
    text: &'a str,
    captures: Captures,
) -> Result<(Direction, Vnum), Box<dyn Error + 'a>> {
    let direction_match = captures.name("direction").unwrap();
    let direction = match &text[direction_match.start()..direction_match.end()] {
        "0" => Direction::North,
        "1" => Direction::East,
        "2" => Direction::South,
        "3" => Direction::West,
        "4" => Direction::Up,
        "5" => Direction::Down,
        dir => return Err(Box::new(InvalidDirection(dir))),
    };
    let destination_match = captures.name("destination").unwrap();
    let destination = u32::from_str(&text[destination_match.start()..destination_match.end()])?;

    Ok((direction, destination))
}

#[derive(Debug)]
struct InvalidDirection<'a>(&'a str);

impl<'a> Display for InvalidDirection<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid direction: {}", self.0)
    }
}

impl<'a> Error for InvalidDirection<'a> {}

fn sort_rooms(rooms: Vec<Room>) -> Vec<Vec<Rc<Room>>> {
    let rooms: Vec<_> = rooms.into_iter().map(|r| Rc::new(r)).collect();
    let all_rooms = {
        let mut hash = HashMap::default();
        for room in &rooms {
            hash.insert(room.vnum, room.clone());
        }
        hash
    };

    let mut floors_hash = HashMap::default();
    for room in rooms {
        let floor = {
            let (floor, _) = floors_hash
                .entry(room.vnum)
                .or_insert_with(|| (0i32, room.clone()));
            *floor
        };
        for (dir, dest_vnum) in &room.exits {
            if floors_hash.contains_key(dest_vnum) {
                continue;
            }
            if let Some(dest) = all_rooms.get(dest_vnum).cloned() {
                match dir {
                    Direction::Up => {
                        floors_hash.insert(*dest_vnum, (floor + 1, dest.clone()));
                    }
                    Direction::Down => {
                        floors_hash.insert(*dest_vnum, (floor - 1, dest.clone()));
                    }
                    _ => {
                        floors_hash.insert(*dest_vnum, (floor, dest.clone()));
                    }
                }
            }
        }
    }

    let mut floors = HashMap::default();
    for (_, (floor_num, dest)) in floors_hash {
        let floor = floors.entry(floor_num).or_insert_with(|| vec![]);
        floor.push(dest);
    }

    floors.into_values().collect()
}
