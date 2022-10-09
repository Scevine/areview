mod connection;
mod position_rooms;

use crate::room::{Room, Sector, Vnum};
use connection::find_connections;
pub use connection::{Connection, Exit};
use fnv::{FnvHashMap, FnvHashSet};
use nannou::prelude::{Rect, Vec2};
use nannou::winit::event::DeviceId;
use position_rooms::position_rooms;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Model {
    square_size: f32,
    pub rooms: Vec<Room>,
    pub locations: Vec<Vec2>,
    pub room_planes: Vec<usize>,
    pub selected: Vec<bool>,
    pub plane_areas: Vec<Rect>,
    pub sectors: Vec<Sector>,
    pub connections: Vec<Connection>,
    pub ui: Ui,
}

impl Model {
    pub fn new(
        square_size: f32,
        all_rooms: FnvHashMap<Vnum, Rc<Room>>,
        grouped_rooms: Vec<Vec<Rc<Room>>>,
    ) -> Self {
        let (plane_areas, all_locations) = position_rooms(&all_rooms, grouped_rooms, square_size);

        let num_rooms = all_locations.len();

        let sectors = all_locations
            .iter()
            .map(|l| l.room.sector)
            .collect::<FnvHashSet<Sector>>()
            .into_iter()
            .collect();
        let rooms: Vec<_> = all_locations.iter().map(|l| (*l.room).clone()).collect();
        let locations = all_locations.iter().map(|l| Vec2::new(l.x, l.y)).collect();
        let room_planes = all_locations.into_iter().map(|l| l.group).collect();

        let indexes_by_vnums = rooms
            .iter()
            .enumerate()
            .map(|(idx, room)| (room.vnum, idx))
            .collect();

        let connections = find_connections(&all_rooms, indexes_by_vnums);

        Model {
            square_size,
            rooms,
            locations,
            room_planes,
            selected: vec![false; num_rooms],
            plane_areas,
            sectors,
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

    pub fn recalculate_guides(&mut self) {
        let locations: Vec<_> = self
            .locations
            .iter()
            .zip(&self.selected)
            .filter_map(|(loc, selected)| if *selected { None } else { Some(loc) })
            .collect();
        let mut xs = Vec::with_capacity(locations.len());
        let mut ys = Vec::with_capacity(locations.len());
        for loc in locations {
            if !xs.contains(&loc.x) {
                xs.push(loc.x)
            }
            if !ys.contains(&loc.y) {
                ys.push(loc.y)
            }
        }
        self.ui.guides = Some(SnapTo { xs, ys });
    }

    pub fn clear_guides(&mut self) {
        self.ui.guides = None;
    }
}

#[derive(Debug, Default)]
pub struct Ui {
    pub device_pressed: Option<DeviceId>,
    pub grabbed: Option<usize>,
    pub grab_origin: Option<Vec2>,
    pub grab_offset: Option<Vec2>,
    pub last_click_device: Option<DeviceId>,
    pub last_click_time: Duration,
    pub guides: Option<SnapTo>,
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

#[derive(Debug)]
pub struct SnapTo {
    pub xs: Vec<f32>,
    pub ys: Vec<f32>,
}
