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
        let pdraw = draw.x_y(fake_pos.0, fake_pos.1);
        for location in &plane.locations {
            let x = location.x * SQUARE_SIZE * 2f32;
            let y = location.y * SQUARE_SIZE * 2f32;
            pdraw
                .rect()
                .x_y(x, y)
                .w_h(SQUARE_SIZE, SQUARE_SIZE)
                .stroke(BLACK)
                .stroke_weight(2f32)
                .color(WHITE);
            pdraw
                .text(&location.room.vnum.to_string())
                .x_y(x, y)
                .color(RED);
        }
        pdraw.ellipse().x_y(0f32, 0f32).radius(5f32).color(BLUE);
        pdraw
            .ellipse()
            .x_y(
                plane.center_x * SQUARE_SIZE * 2f32,
                plane.center_y * SQUARE_SIZE * 2f32,
            )
            .radius(5f32)
            .color(GREEN);
    }
    draw.to_frame(app, &frame).unwrap();
}
