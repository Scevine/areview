use crate::room::{Direction, Room, Vnum};
use fnv::FnvHashMap;
use nannou::prelude::{Rect, Vec2};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Location {
    pub x: f32,
    pub y: f32,
    pub room: Rc<Room>,
    pub group: usize,
}

pub fn position_rooms(
    all_rooms: &FnvHashMap<Vnum, Rc<Room>>,
    planes: Vec<Vec<Rc<Room>>>,
    square_size: f32,
) -> (Vec<Rect>, Vec<Location>) {
    let mut grouped_locations: Vec<_> = planes
        .into_iter()
        .enumerate()
        .map(|(index, plane)| position_rooms_in_plane(all_rooms, plane, square_size, index))
        .collect();

    let shift_groups_by = grouped_locations.len() as isize / 2isize;
    for (i, (plane, locations)) in grouped_locations.iter_mut().enumerate() {
        // temporarily arrange the groups of rooms
        let mut pos = i as f32 - shift_groups_by as f32;
        pos *= 150f32;

        let (x, y) = plane.x_y();
        *plane = Rect::from_xy_wh(Vec2::new(pos, pos), plane.wh() + square_size);

        for loc in locations {
            loc.x += pos - x;
            loc.y += pos - y;
        }
    }

    let plane_areas: Vec<_> = grouped_locations.iter().map(|(area, _)| *area).collect();

    let mut all_locations: Vec<_> = grouped_locations
        .into_iter()
        .map(|(_, ls)| ls)
        .flatten()
        .collect();
    all_locations.sort_by(|a, b| a.room.vnum.cmp(&b.room.vnum));
    (plane_areas, all_locations)
}

fn position_rooms_in_plane(
    all_rooms: &FnvHashMap<Vnum, Rc<Room>>,
    plane: Vec<Rc<Room>>,
    square_size: f32,
    group: usize,
) -> (Rect, Vec<Location>) {
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

    let area = {
        let mut max = Vec2::default();
        let mut min = Vec2::default();
        for loc in &locations {
            max.x = max.x.max(loc.x);
            max.y = max.y.max(loc.y);
            min.x = min.x.min(loc.x);
            min.y = min.y.min(loc.y);
        }

        // The rect intersects with the room centers, not their outermost edges
        Rect::from_corners(min, max)
    };

    (area, locations)
}
