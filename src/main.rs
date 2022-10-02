mod room;

use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::Path;

use room::{parse_rooms, Room};

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        panic!("No path to area file supplied!");
    });
    let rooms = load_area(&path).unwrap();
    println!("{:?}", rooms);
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
