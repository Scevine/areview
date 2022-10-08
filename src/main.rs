mod draw;
mod model;
mod parser;
mod room;

use crate::draw::{draw_connections, draw_legend, draw_rooms, LabelColor};
use crate::model::Exit;
use crate::room::Direction;
use model::{Connection, Model};
use nannou::event::ElementState;
use nannou::prelude::*;
use nannou::winit::event::DeviceEvent;
use parser::load_area;

fn main() {
    nannou::app(model).event(event).simple_window(view).loop_mode(LoopMode::Wait).run();
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

fn apply_grab(model: &mut Model) {
    if let Some(grab_offset) = model.ui.grab_offset.take() {
        for (location, selected) in model.locations.iter_mut().zip(&model.selected) {
            if *selected {
                *location += grab_offset;
            }
        }
    }
    model.ui.grab_origin = None;
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
                if !model.selected[room_idx] {
                    apply_grab(model);
                }
                model.selected[room_idx] = true;
                if is_double_click {
                    model.select_all_in_plane(model.room_planes[room_idx]);
                }
                model.ui.grab_origin = Some(model.locations[room_idx]);
            } else {
                apply_grab(model);
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
        Event::DeviceEvent(device_id, DeviceEvent::MouseMotion { .. }) => {
            if let Some(id) = model.ui.device_pressed {
                if id == device_id {
                    if let Some(grab_origin) = model.ui.grab_origin {
                        let mut offset = Vec2::new(app.mouse.x, app.mouse.y) - grab_origin;
                        // TODO: apply restrict_axis immediately when shift is pressed/released too, not just on mouse move
                        let restrict_axis = app.keys.mods.shift();
                        if restrict_axis {
                            let closer_to_x_axis = f32::abs(offset.x) < f32::abs(offset.y);
                            if closer_to_x_axis {
                                offset.x = 0f32;
                            } else {
                                offset.y = 0f32;
                            }
                        }
                        model.ui.grab_offset = Some(offset);
                    }
                }
            }
        }
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw_legend(&draw.xy(app.window_rect().top_left()));

    draw_connections(&draw, model);

    draw_rooms(&draw, model);

    draw.to_frame(app, &frame).unwrap();
}
