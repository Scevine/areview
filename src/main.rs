mod model;
mod parser;
mod room;

use model::Model;
use nannou::event::ElementState;
use nannou::prelude::*;
use nannou::winit::event::DeviceEvent;
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

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::DeviceEvent(
            device_id,
            DeviceEvent::Button {
                button: 1,
                state: ElementState::Pressed,
            },
        ) => {
            model.button_pressed = Some(device_id);

            // Did it encounter a location group?
            println!(
                "Device pressed button 1 at {:?} {:?}",
                app.mouse.x, app.mouse.y
            );

            let mut hit = None;
            for ((idx, loc), &group) in model.locations.iter().enumerate().zip(&model.room_planes) {
                let window_pos = model.groups[group].0 + *loc;
                let half_square_size = model.square_size() * 0.5;
                if app.mouse.x + half_square_size > window_pos.x
                    && app.mouse.x - half_square_size < window_pos.x
                    && app.mouse.y + half_square_size > window_pos.y
                    && app.mouse.y - half_square_size < window_pos.y
                {
                    let room = &model.rooms[idx];
                    println!("room {} hit", &room.vnum);
                    hit = Some(idx);
                }
            }
            model.selected = hit;
        }
        Event::DeviceEvent(
            device_id,
            DeviceEvent::Button {
                button: 1,
                state: ElementState::Released,
            },
        ) => {
            match model.button_pressed {
                Some(id) if id == device_id => {
                    model.button_pressed = None;
                }
                _ => {}
            }
            // println!("Device released button 1");
        }
        _ => {}
    }
}

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
