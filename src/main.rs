use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::path::Path;
use std::str::FromStr;

use regex::{Match, Regex};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        panic!("No path to area file supplied!");
    });
    let rooms = load_area(&path).unwrap();
    println!("{:?}", rooms);
}

#[derive(Debug, Copy, Clone)]
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

fn load_area(path: &dyn AsRef<Path>) -> Result<Vec<Room>, Box<dyn std::error::Error>> {
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

impl std::error::Error for NoRooms {}

fn parse_rooms(text: &str) -> Vec<Room> {
    let room_split_regex = Regex::new(r"(?m)^#(\d+)").unwrap();
    let room_matches: Vec<Match> = room_split_regex.find_iter(text).collect();
    let mut rooms = vec![];
    if room_matches.is_empty() {
        return rooms;
    }

    let room_regex = Regex::new(r"(?mx)\A\s*
        (?P<name>[^~]*)~
        [^~]*~\s*
        \d+\s+(?P<flags>[\d|]+)\s+(?P<sector>\d+)\s*").unwrap();

    for matches in room_matches.windows(2) {
        let m = matches.get(0).unwrap();
        let next = matches.get(1).unwrap();

        match parse_room(text, m.start(), m.end(), next.start(), &room_regex) {
            Ok(room) => rooms.push(room),
            Err(e) => eprintln!("{e}"),
        }
    }

    let last_match = room_matches.iter().last().unwrap();
    match parse_room(text, last_match.start(), last_match.end(), text.len(), &room_regex) {
        Ok(room) => rooms.push(room),
        Err(e) => eprintln!("{e}"),
    }

    rooms
}

fn parse_room<'a>(text: &'a str, vnum_start: usize, vnum_end: usize, text_end: usize, regex: &Regex) -> Result<Room, Box<dyn std::error::Error + 'a>> {
    let vnum_text = &text[vnum_start + 1 .. vnum_end];
    let vnum = u16::from_str(vnum_text)?;
    let room_body = &text[vnum_end .. text_end];
    let captures = regex.captures(room_body)
        .ok_or(InvalidRoomBody { body: room_body })?;

    let name_match = captures.name("name").unwrap();
    let name = room_body[name_match.start()..name_match.end()].to_string();
    let exits = HashMap::new();

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

impl<'a> std::error::Error for InvalidRoomBody<'a> {}