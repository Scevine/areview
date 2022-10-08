mod parse_rooms;
mod sort_rooms;

use crate::room::{Room, SimpleConnection, Vnum};
use fnv::{FnvHashMap, FnvHashSet};
use std::error::Error;
use std::path::Path;
use std::rc::Rc;

pub fn load_area(
    path: &dyn AsRef<Path>,
) -> Result<
    (
        FnvHashMap<Vnum, Rc<Room>>,
        Vec<Vec<Rc<Room>>>,
        FnvHashSet<SimpleConnection>,
    ),
    Box<dyn Error>,
> {
    let file = std::fs::read_to_string(path)?;
    let rooms = parse_rooms::parse_rooms(&file)?;
    Ok(sort_rooms::sort_rooms(rooms))
}
