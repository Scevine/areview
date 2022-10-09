mod draw_connection;
mod draw_legend;
mod draw_room;

use crate::Model;
use nannou::prelude::*;

pub use draw_connection::draw_connections;
pub use draw_legend::draw_legend;
pub use draw_room::draw_rooms;

pub struct LabelColor {
    pub background: Rgb8,
    pub foreground: Rgb8,
}

impl LabelColor {
    #[inline]
    pub fn light(background: Rgb8) -> Self {
        LabelColor {
            background,
            foreground: BLACK,
        }
    }
    #[inline]
    pub fn dark(background: Rgb8) -> Self {
        LabelColor {
            background,
            foreground: WHITE,
        }
    }
}

pub fn location_of(model: &Model, index: usize) -> Vec2 {
    if model.selected[index] {
        model.locations[index] + model.ui.grab_offset.unwrap_or_default()
    } else {
        model.locations[index]
    }
}
