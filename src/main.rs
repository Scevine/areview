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
    Model::new(all_rooms, by_plane, connections)
}

fn event(_app: &App, _model: &mut Model, _event: Event) {}

const SQUARE_SIZE: f32 = 30f32;

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let fake_positions = &[(-250f32, -250f32), (0f32, 0f32), (250f32, 250f32)];
    for (plane, fake_pos) in model.room_planes.iter().zip(fake_positions.iter()) {
        for location in &plane.locations {
            let x = fake_pos.0 + location.x * SQUARE_SIZE * 2f32;
            let y = fake_pos.1 + location.y * SQUARE_SIZE * 2f32;
            draw.rect()
                .x_y(x, y)
                .w_h(SQUARE_SIZE, SQUARE_SIZE)
                .color(BLACK);
            draw.text(&location.room.vnum.to_string())
                .x_y(x, y)
                .color(RED);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
