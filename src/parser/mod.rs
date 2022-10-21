mod connection;
mod parse_rooms;
mod sort_rooms;

use crate::model::{Room, Vnum};
pub use connection::{Connection, Exit};
use fnv::FnvHashMap;
pub use sort_rooms::Location;
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

pub struct ParsedArea {
    pub all_rooms: FnvHashMap<Vnum, (Rc<Room>, usize)>,
    pub grouped_rooms: Vec<Vec<Location>>,
    pub connections: Vec<Connection>,
}

pub fn load_area(path: &dyn AsRef<Path>) -> Result<ParsedArea, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    let rooms = parse_rooms::parse_rooms(&file)?;

    let rooms: Vec<_> = rooms.into_iter().map(Rc::new).collect();
    let connections = connection::find_connections(&rooms);

    let (all_rooms, grouped_rooms) = sort_rooms::sort_rooms(rooms);

    Ok(ParsedArea {
        all_rooms,
        grouped_rooms,
        connections,
    })
}
