mod model;
mod parser;
mod room;

use model::Model;
use nannou::prelude::*;
use parser::load_area;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No path to area file supplied!");
        std::process::exit(1);
    });
    let (all_rooms, by_plane, connections) = match load_area(&path) {
        Ok(rooms) => rooms,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    Model::new(30f32, all_rooms, by_plane, connections)
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

const SQUARE_SIZE: f32 = 30f32;
const GRID_SIZE: f32 = SQUARE_SIZE * 2f32;

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // For each room
    for ((room, location), &plane) in model
        .rooms
        .iter()
        .zip(&model.locations)
        .zip(&model.room_planes)
    {
        let plane = &model.groups[plane];

        let rdraw = draw
            .x_y(plane.0.x, plane.0.y) // translate to group's location for now
            .x_y(location.x, location.y);

        rdraw
            .rect()
            .w_h(model.square_size(), model.square_size())
            .stroke(BLACK)
            .stroke_weight(2f32)
            .color(WHITE);
        rdraw.text(&room.vnum.to_string()).color(RED);
    }
    draw.to_frame(app, &frame).unwrap();
}
