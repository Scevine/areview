use crate::room::{Connection, Direction, Room, Vnum};
use fnv::{FnvHashMap, FnvHashSet};
use nannou::prelude::Vec2;
use nannou::winit::event::DeviceId;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Clone, Default)]
pub struct Model {
    square_size: f32,
    pub rooms: Vec<Room>,
    pub locations: Vec<Vec2>,
    pub room_planes: Vec<usize>,
    pub selected: Vec<bool>,
    pub groups: Vec<Group>,
    pub connections: FnvHashSet<Connection>,
    pub ui: Ui,
}

impl Model {
    pub fn new(
        square_size: f32,
        all_rooms: FnvHashMap<Vnum, Rc<Room>>,
        grouped_rooms: Vec<Vec<Rc<Room>>>,
        connections: FnvHashSet<Connection>,
    ) -> Self {
        let grouped_locations = position_rooms(&all_rooms, grouped_rooms, square_size);
        let mut groups: Vec<_> = grouped_locations
            .iter()
            .map(|(center, _)| Group(*center))
            .collect();
        let mut all_locations: Vec<_> = grouped_locations
            .into_iter()
            .map(|(_, ls)| ls)
            .flatten()
            .collect();
        all_locations.sort_by(|a, b| a.room.vnum.cmp(&b.room.vnum));
        let num_rooms = all_locations.len();

        let rooms = all_locations.iter().map(|l| (*l.room).clone()).collect();
        let locations = all_locations.iter().map(|l| Vec2::new(l.x, l.y)).collect();
        let room_planes = all_locations.into_iter().map(|l| l.group).collect();

        let shift_groups_by = groups.capacity() as isize / 2isize;
        for (i, group) in groups.iter_mut().enumerate() {
            let mut pos = i as f32 - shift_groups_by as f32;
            pos *= 150f32;
            group.0 = pos - group.0;
        }

        Model {
            square_size,
            rooms,
            locations,
            room_planes,
            selected: vec![false; num_rooms],
            groups,
            connections,
            ..Default::default()
        }
    }

    #[inline]
    pub fn square_size(&self) -> f32 {
        self.square_size
    }

    pub fn select_all_in_plane(&mut self, group: usize) {
        for (&plane, selected) in self.room_planes.iter().zip(&mut self.selected) {
            if plane == group {
                *selected = true;
            }
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Ui {
    pub device_pressed: Option<DeviceId>,
    pub grabbed: Option<usize>,
    pub last_click_device: Option<DeviceId>,
    pub last_click_time: Duration,
}

const DOUBLE_CLICK_THRESHOLD: Duration = Duration::from_millis(250);

impl Ui {
    pub fn is_double_click(&self, device: DeviceId, now: Duration) -> bool {
        if let Some(previous_device) = self.device_pressed {
            (previous_device == device) && now - self.last_click_time < DOUBLE_CLICK_THRESHOLD
        } else {
            false
        }
    }
}

#[derive(Debug, Clone)]
struct Location {
    x: f32,
    y: f32,
    room: Rc<Room>,
    group: usize,
}

#[derive(Debug, Clone, Default)]
pub struct Group(pub Vec2);

fn position_rooms(
    all_rooms: &FnvHashMap<Vnum, Rc<Room>>,
    planes: Vec<Vec<Rc<Room>>>,
    square_size: f32,
) -> Vec<(Vec2, Vec<Location>)> {
    planes
        .into_iter()
        .enumerate()
        .map(|(index, plane)| position_rooms_in_plane(all_rooms, plane, square_size, index))
        .collect()
}

fn position_rooms_in_plane(
    all_rooms: &FnvHashMap<Vnum, Rc<Room>>,
    plane: Vec<Rc<Room>>,
    square_size: f32,
    group: usize,
) -> (Vec2, Vec<Location>) {
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

    let center = {
        let mut max = Vec2::default();
        let mut min = Vec2::default();
        for loc in &locations {
            max.x = max.x.max(loc.x);
            max.y = max.y.max(loc.y);
            min.x = min.x.min(loc.x);
            min.y = min.y.min(loc.y);
        }
        (max + min) / 2f32
    };

    (center, locations)
}
