mod room;

use std::error::Error;
use std::path::Path;
use std::rc::Rc;

use room::{parse_rooms, Room};

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No path to area file supplied!");
        std::process::exit(1);
    });
    let rooms_by_floor = match load_area(&path) {
        Ok(rooms) => rooms,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    println!("{:?}", rooms_by_floor);
    Ok(())
}

fn load_area(path: &dyn AsRef<Path>) -> Result<Vec<Vec<Rc<Room>>>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    parse_rooms(&file)
}
