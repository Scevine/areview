mod room;

use std::error::Error;
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
    parse_rooms(&file)
}
