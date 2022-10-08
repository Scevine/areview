use crate::{LabelColor, Model};
use nannou::color::named::BLACK;
use nannou::Draw;

pub fn draw_rooms(draw: &Draw, model: &Model) {
    for ((room, location), &selected) in model
        .rooms
        .iter()
        .zip(&model.locations)
        .zip(&model.selected)
    {
        let mut rdraw = draw.xy(*location);
        if let Some(grab_offset) = model.ui.grab_offset {
            if selected {
                rdraw = rdraw.xy(grab_offset);
            }
        }

        if selected {
            rdraw
                .rect()
                .w_h(model.square_size(), model.square_size())
                .no_fill()
                .stroke(nannou::color::rgb_u32(0xf04e98))
                .stroke_weight(10f32)
                .finish();
        }

        let LabelColor {
            background,
            foreground,
        } = room.sector.color();

        rdraw
            .rect()
            .w_h(model.square_size(), model.square_size())
            .color(background);
        rdraw
            .rect()
            .w_h(model.square_size(), model.square_size())
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(2f32)
            .finish();
        rdraw.text(&room.string_vnum).color(foreground);
    }
}
