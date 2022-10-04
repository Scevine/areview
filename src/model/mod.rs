mod group;
mod location;

use crate::room::{Connection, Direction, Room, Vnum};
pub use group::Group;
pub use location::Location;
use fnv::{FnvHashMap, FnvHashSet};
use nannou::winit::event::DeviceId;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct Model {
    pub all_rooms: FnvHashMap<Vnum, Rc<Room>>,
    pub room_planes: Vec<Group>,
    pub connections: FnvHashSet<Connection>,
}

impl Model {
    pub fn new(
        all_rooms: FnvHashMap<Vnum, Rc<Room>>,
        room_planes: Vec<Vec<Rc<Room>>>,
        connections: FnvHashSet<Connection>,
    ) -> Self {
        let room_planes = position_rooms(&all_rooms, room_planes);
        Model {
            all_rooms,
            room_planes,
            connections,
        }
    }
}

// #[derive(Debug, Copy, Clone, Eq, PartialEq)]
// pub enum Endpoint {
//     Open(Vnum),
//     External(Vnum),
// }
//
// #[derive(Debug, Copy, Clone)]
// pub struct Connection {
//     left: Endpoint,
//     right: Endpoint,
// }
//
// impl PartialEq<Self> for Connection {
//     fn eq(&self, other: &Self) -> bool {
//         (self.left == other.left && self.right == other.right)
//             || (self.left == other.right && self.right == other.left)
//     }
// }
//
// impl Eq for Connection {}

fn position_rooms(
    all_rooms: &FnvHashMap<Vnum, Rc<Room>>,
    planes: Vec<Vec<Rc<Room>>>,
) -> Vec<Group> {
    planes
        .into_iter()
        .map(|plane| position_rooms_in_plane(all_rooms, plane))
        .collect()
}

fn position_rooms_in_plane(all_rooms: &FnvHashMap<Vnum, Rc<Room>>, plane: Vec<Rc<Room>>) -> Group {
    let mut locations: Vec<Location> = Vec::with_capacity(plane.len());

    let mut to_visit = std::collections::VecDeque::new();
    to_visit.push_back(Location {
        x: 0f32,
        y: 0f32,
        room: plane[0].clone(),
    });
    while !to_visit.is_empty() {
        let loc = to_visit.pop_front().unwrap();
        for (dir, vnum) in &loc.room.exits {
            if to_visit.iter().any(|l| l.room.vnum == *vnum) {
                continue;
            }
            if locations.iter().any(|l| l.room.vnum == *vnum) {
                continue;
            }
            if let Some(dest) = all_rooms.get(vnum) {
                let (x, y) = match dir {
                    Direction::North => (loc.x, loc.y + 1f32),
                    Direction::East => (loc.x + 1f32, loc.y),
                    Direction::South => (loc.x, loc.y - 1f32),
                    Direction::West => (loc.x - 1f32, loc.y),
                    Direction::Up | Direction::Down => {
                        continue;
                    }
                };
                to_visit.push_back(Location {
                    x,
                    y,
                    room: dest.clone(),
                });
            }
        }
        locations.push(loc);
    }

    // At this point, all rooms in `plane` should have been walked over, since a plane should consist
    // 100% of rooms that are connected together in NSEW directions.
    for room in plane
        .iter()
        .filter(|r| locations.iter().find(|l| l.room.vnum == r.vnum).is_none())
    {
        eprintln!("Room {} was not positioned as a location!", room.vnum);
    }

    Group::new(locations)
}
