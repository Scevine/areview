use once_cell::sync::Lazy;
use regex::{Captures, Match, Regex};
use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
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

#[derive(Debug)]
pub struct Room {
    pub name: String,
    pub vnum: u16,
    pub exits: HashMap<Direction, u16>,
}

pub fn parse_rooms(text: &str) -> Vec<Room> {
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
    let vnum = u16::from_str(vnum_text)?;
    let room_body = &text[vnum_end..text_end];
    let captures = ROOM_REGEX
        .captures(room_body)
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

fn parse_door<'a>(
    text: &'a str,
    captures: Captures,
) -> Result<(Direction, u16), Box<dyn Error + 'a>> {
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
