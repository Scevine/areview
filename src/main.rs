mod model;
mod parser;
mod room;

use model::{Connection, Model};
use nannou::event::ElementState;
use nannou::prelude::*;
use nannou::winit::event::DeviceEvent;
use parser::load_area;
use room::Sector;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

fn model(_app: &App) -> Model {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No path to area file supplied!");
        std::process::exit(1);
    });
    let (all_rooms, by_plane) = match load_area(&path) {
        Ok(rooms) => rooms,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    Model::new(30f32, all_rooms, by_plane)
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
            model.ui.device_pressed = Some(device_id);
            let is_double_click = model
                .ui
                .is_double_click(device_id, app.duration.since_start);
            model.ui.last_click_device = Some(device_id);
            model.ui.last_click_time = app.duration.since_start;

            let mut grabbed_room = None;
            for (idx, loc) in model.locations.iter().enumerate() {
                let half_square_size = model.square_size() * 0.5;
                if app.mouse.x + half_square_size > loc.x
                    && app.mouse.x - half_square_size < loc.x
                    && app.mouse.y + half_square_size > loc.y
                    && app.mouse.y - half_square_size < loc.y
                {
                    grabbed_room = Some(idx);
                }
            }
            model.ui.grabbed = grabbed_room;

            // Handle selecting rooms
            if let Some(room_idx) = grabbed_room {
                model.selected[room_idx] = true;
                if is_double_click {
                    model.select_all_in_plane(model.room_planes[room_idx]);
                }
            } else {
                model.selected.fill(false);
            }
        }
        Event::DeviceEvent(
            device_id,
            DeviceEvent::Button {
                button: 1,
                state: ElementState::Released,
            },
        ) => match model.ui.device_pressed {
            Some(id) if id == device_id => {
                model.ui.device_pressed = None;
                model.ui.grabbed = None;
            }
            _ => {}
        },
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    // DEBUG for each group
    for plane in &model.plane_areas {
        draw.rect()
            .x_y(plane.x.middle(), plane.y.middle())
            .width(plane.x.len())
            .height(plane.y.len())
            .no_fill()
            .stroke(RED)
            .stroke_weight(2f32)
            .finish();
    }

    // Draw connections
    for connection in &model.connections {
        match connection {
            Connection::TwoWay { from, to, door } => {
                let from_room = model.locations[from.index];
                let to_room = model.locations[to.index];
                draw.line().start(from_room).end(to_room);
            }
            _ => {}
        }
    }

    // Draw rooms
    for ((room, location), &selected) in model
        .rooms
        .iter()
        .zip(&model.locations)
        .zip(&model.selected)
    {
        let rdraw = draw.xy(*location);

        if selected {
            rdraw
                .rect()
                .w_h(model.square_size(), model.square_size())
                .no_fill()
                .stroke(nannou::color::rgb_u32(0xf04e98))
                .stroke_weight(10f32)
                .finish();
        }

        let room_color = match room.sector {
            Sector::Inside => GREEN,
            Sector::City => GREEN,
            Sector::Field => GREEN,
            Sector::Forest => GREEN,
            Sector::Hills => GREEN,
            Sector::Mountain => GREEN,
            Sector::WaterSwim => CYAN,
            Sector::WaterNoswim => CYAN,
            Sector::House => GREEN,
            Sector::Air => WHITE,
            Sector::Desert => GREEN,
            Sector::Underwater => CYAN,
            Sector::OnBottom => CYAN,
            Sector::RogueGuild => GREEN,
        };

        rdraw
            .rect()
            .w_h(model.square_size(), model.square_size())
            .color(room_color);
        rdraw
            .rect()
            .w_h(model.square_size(), model.square_size())
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(2f32)
            .finish();
        rdraw.text(&room.string_vnum).color(RED);
    }
    draw.to_frame(app, &frame).unwrap();
}
