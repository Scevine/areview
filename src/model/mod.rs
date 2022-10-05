use crate::room::{Connection, Direction, Room, Vnum};
use fnv::{FnvHashMap, FnvHashSet};
use nannou::prelude::Vec2;
use nannou::winit::event::DeviceId;
use std::rc::Rc;

#[derive(Debug, Clone, Default)]
pub struct Model {
    square_size: f32,
    pub rooms: Vec<Room>,
    pub locations: Vec<Vec2>,
    pub room_planes: Vec<usize>,
    pub connections: FnvHashSet<Connection>,
    pub button_pressed: Option<DeviceId>,
}

impl Model {
    pub fn new(
        square_size: f32,
        all_rooms: FnvHashMap<Vnum, Rc<Room>>,
        room_planes: Vec<Vec<Rc<Room>>>,
        connections: FnvHashSet<Connection>,
    ) -> Self {
        let mut all_locations = position_rooms(&all_rooms, room_planes, square_size);
        all_locations.sort_by(|a, b| a.room.vnum.cmp(&b.room.vnum));

        let rooms = all_locations.iter().map(|l| (*l.room).clone()).collect();
        let locations = all_locations.iter().map(|l| Vec2::new(l.x, l.y)).collect();
        let room_planes = all_locations.iter().map(|l| l.group).collect();

        Model {
            square_size,
            rooms,
            locations,
            room_planes,
            connections,
            ..Default::default()
        }
    }

    #[inline]
    pub fn square_size(&self) -> f32 {
        self.square_size
    }
}

#[derive(Debug, Clone)]
struct Location {
    x: f32,
    y: f32,
    room: Rc<Room>,
    group: usize,
}

fn position_rooms(
    all_rooms: &FnvHashMap<Vnum, Rc<Room>>,
    planes: Vec<Vec<Rc<Room>>>,
    square_size: f32,
) -> Vec<Location> {
    planes
        .into_iter()
        .enumerate()
        .flat_map(|(group, plane)| position_rooms_in_plane(all_rooms, plane, square_size, group))
        .collect()
}

fn position_rooms_in_plane(
    all_rooms: &FnvHashMap<Vnum, Rc<Room>>,
    plane: Vec<Rc<Room>>,
    square_size: f32,
    group: usize,
) -> Vec<Location> {
    let mut locations: Vec<Location> = Vec::with_capacity(plane.len());

    let mut to_visit = std::collections::VecDeque::new();
    to_visit.push_back(Location {
        x: 0f32,
        y: 0f32,
        room: plane[0].clone(),
        group,
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
                    group,
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

    for location in &mut locations {
        location.x *= square_size * 2.0;
        location.y *= square_size * 2.0;
    }

    locations
}
