use once_cell::sync::Lazy;
use regex::{Captures, Match, Regex};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        panic!("No path to area file supplied!");
    });
    let rooms = load_area(&path).unwrap();
    println!("{:?}", rooms);
}

static ROOM_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?mx)\A\s*
        (?P<name>[^~]*)~
        [^~]*~\s*
        \d+\s+(?P<flags>[\d|]+)\s+(?P<sector>\d+)\s*").unwrap()
});

static DOOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?imsx)
        ^D(?P<direction>\d+)
        [^~]*~
        [^~]*~\s*
        (?P<locks>\d+)\s+\d+\s+(?P<destination>\d+)").unwrap()
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

#[derive(Debug)]
struct Room {
    name: String,
    vnum: u16,
    exits: HashMap<Direction, u16>,
}

fn load_area(path: &dyn AsRef<Path>) -> Result<Vec<Room>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;

    let room_section_regex = Regex::new(r"(?ims)^#ROOMS$(.*?)^#0$").unwrap();

    // TODO: write a PR to regex to let String be indexed by match
    let room_section = room_section_regex.captures(&file).ok_or(NoRooms)?;

    let section_match = room_section.get(1).unwrap();
    let section_text = &file[section_match.start()..section_match.end()];
    Ok(parse_rooms(section_text))
}

#[derive(Debug)]
struct NoRooms;

impl Display for NoRooms {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "No #ROOMS section detected in file")
    }
}

impl Error for NoRooms {}

fn parse_rooms(text: &str) -> Vec<Room> {
    let room_split_regex = Regex::new(r"(?m)^#(\d+)").unwrap();
    let room_matches: Vec<Match> = room_split_regex.find_iter(text).collect();
    let mut rooms = vec![];
    if room_matches.is_empty() {
        return rooms;
    }

    for matches in room_matches.windows(2) {
        let m = matches.get(0).unwrap();
        let next = matches.get(1).unwrap();

        match parse_room(text, m.start(), m.end(), next.start()) {
            Ok(room) => rooms.push(room),
            Err(e) => eprintln!("{e}"),
        }
    }

    let last_match = room_matches.iter().last().unwrap();
    match parse_room(text, last_match.start(), last_match.end(), text.len()) {
        Ok(room) => rooms.push(room),
        Err(e) => eprintln!("{e}"),
    }

    rooms
}

fn parse_room<'a>(text: &'a str, vnum_start: usize, vnum_end: usize, text_end: usize) -> Result<Room, Box<dyn Error + 'a>> {
    let vnum_text = &text[vnum_start + 1 .. vnum_end];
    let vnum = u16::from_str(vnum_text)?;
    let room_body = &text[vnum_end .. text_end];
    let captures = ROOM_REGEX.captures(room_body)
        .ok_or(InvalidRoomBody { body: room_body })?;

    let name_match = captures.name("name").unwrap();
    let name = room_body[name_match.start()..name_match.end()].to_string();

    let captures_match = captures.get(0).unwrap();
    let exits = parse_doors(&room_body[captures_match.end()..]);

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

fn parse_doors(text: &str) -> HashMap<Direction, u16> {
    let mut exits = HashMap::new();

    for captures in DOOR_REGEX.captures_iter(text) {
        match parse_door(text, captures) {
            Ok((direction, destination)) => {
                exits.insert(direction, destination);
            }
            Err(e) => eprintln!("{e}"),
        }
    }

    exits
}

fn parse_door<'a>(text: &'a str, captures: Captures) -> Result<(Direction, u16), Box<dyn Error + 'a>> {
    let direction_match = captures.name("direction").unwrap();
    let direction = match &text[direction_match.start()..direction_match.end()] {
        "0" => Direction::North,
        "1" => Direction::East,
        "2" => Direction::South,
        "3" => Direction::West,
        "4" => Direction::Up,
        "5" => Direction::Down,
        dir => return Err(Box::new(InvalidDirection(dir)))
    };
    let destination_match = captures.name("destination").unwrap();
    let destination = u16::from_str(&text[destination_match.start()..destination_match.end()])?;

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