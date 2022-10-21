use crate::model::Room;
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
    planes: Vec<Vec<crate::parser::Location>>,
    square_size: f32,
) -> (Vec<Rect>, Vec<Location>) {
    let mut grouped_locations: Vec<_> = planes
        .into_iter()
        .enumerate()
        .map(|(index, plane)| position_rooms_in_plane(plane, square_size, index))
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
    plane: Vec<crate::parser::Location>,
    square_size: f32,
    group: usize,
) -> (Rect, Vec<Location>) {
    let locations: Vec<_> = plane
        .into_iter()
        .map(|loc| Location {
            x: loc.x as f32 * square_size * 2.0,
            y: loc.y as f32 * square_size * 2.0,
            room: loc.room,
            group,
        })
        .collect();

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
