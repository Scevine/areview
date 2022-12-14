mod draw;
mod model;
mod parser;

use crate::draw::{draw_connections, draw_legend, draw_rooms, LabelColor};
use crate::model::{Connection, Direction, Exit, Model};
use nannou::event::ElementState;
use nannou::prelude::*;
use nannou::winit::event::DeviceEvent;
use parser::{load_area, ParseRuleError, Rule};

fn main() {
    nannou::app(model)
        .event(event)
        .view(view)
        .loop_mode(LoopMode::Wait)
        .run();
}

fn model(app: &App) -> Model {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No path to area file supplied!");
        std::process::exit(1);
    });
    let rules: Vec<_> = std::env::args()
        .skip(2)
        .filter_map(|arg| match Rule::try_from(arg.as_str()) {
            Ok(rule) => Some(rule),
            Err(ParseRuleError::UnknownRule) => None,
            Err(e) => {
                eprintln!("Error parsing {}, {:?}", &arg, e);
                None
            }
        })
        .collect();

    let area = match load_area(&path, rules) {
        Ok(area) => area,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let _ = app
        .new_window()
        .title("Avatar Area Visualizer")
        .build()
        .unwrap();

    Model::new(30f32, area)
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

fn apply_grab_to_room(model: &mut Model, idx: usize) {
    if let Some(grab_offset) = model.ui.grab_offset {
        model.locations[idx] += grab_offset;
    }
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
            for (idx, &loc) in model.locations.iter().enumerate() {
                let loc = if model.selected[idx] {
                    loc + model.ui.grab_offset.unwrap_or(Vec2::default())
                } else {
                    loc
                };
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
                if app.keys.mods.ctrl() {
                    // If holding ctrl, toggle room selection on and off
                    if model.selected[room_idx] {
                        apply_grab_to_room(model, room_idx);
                        model.selected[room_idx] = false;
                    } else {
                        apply_grab(model);
                        model.selected[room_idx] = true;
                    }
                    model.recalculate_guides();
                } else {
                    // If not holding ctrl, select only one at a time
                    // But also don't clear selection if clicked room is already selected
                    if !model.selected[room_idx] {
                        apply_grab(model);
                        model.selected.fill(false);
                        model.selected[room_idx] = true;
                        model.recalculate_guides();
                    }
                }

                if is_double_click {
                    model.select_all_in_plane(model.room_planes[room_idx]);
                    model.recalculate_guides();
                }
                model.ui.grab_origin = Some(model.locations[room_idx]);
            } else {
                if !app.keys.mods.ctrl() {
                    apply_grab(model);
                    model.selected.fill(false);
                    model.clear_guides();
                }
            }
        }
        Event::DeviceEvent(
            _,
            DeviceEvent::Button {
                button: 3,
                state: ElementState::Pressed,
            },
        ) => {
            // FIXME is rightclick ALWAYS button 3, or is it device specific, and is it gonna fuck me up when someone uses a different mouse?
            model.ui.grab_offset = None;
            model.ui.grab_origin = None;
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
                    // If we're grabbin', set the grab offset
                    if let Some(grab_origin) = model.ui.grab_origin {
                        let mut offset = Vec2::new(app.mouse.x, app.mouse.y) - grab_origin;

                        // Snap to closest guide when current pos is within range
                        // FIXME: once a room gets on a guide axis, it can't go off; disable until fixed
                        if app.keys.mods.shift() {
                            let (guide_x, guide_y) = find_closest_guides(app, model);
                            if let Some(x) = guide_x {
                                offset.x = x - grab_origin.x;
                            }
                            if let Some(y) = guide_y {
                                offset.y = y - grab_origin.y;
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

    draw_legend(&draw.xy(app.window_rect().top_left()), &model.sectors);

    if app.keys.mods.shift() {
        draw_closest_guide(app, &draw, model, app.window_rect());
    }

    draw_connections(&draw, model);

    draw_rooms(&draw, model);

    draw.to_frame(app, &frame).unwrap();
}

const SNAP_TO_THRESHOLD: f32 = 5.0;

fn find_closest_guides(app: &App, model: &Model) -> (Option<f32>, Option<f32>) {
    let current_x = app.mouse.x;
    let current_y = app.mouse.y;
    let snap_to = match model.ui.guides.as_ref() {
        Some(s) => s,
        None => return (None, None),
    };

    let (closest_x, dist_x) =
        snap_to
            .xs
            .iter()
            .fold((f32::INFINITY, f32::INFINITY), |closest, x| {
                let dist = current_x - x;
                if f32::abs(dist) < f32::abs(closest.1) {
                    (*x, dist)
                } else {
                    closest
                }
            });
    let (closest_y, dist_y) =
        snap_to
            .ys
            .iter()
            .fold((f32::INFINITY, f32::INFINITY), |closest, y| {
                let dist = current_y - y;
                if f32::abs(dist) < f32::abs(closest.1) {
                    (*y, dist)
                } else {
                    closest
                }
            });
    let x = if dist_x < SNAP_TO_THRESHOLD {
        Some(closest_x)
    } else {
        None
    };
    let y = if dist_y < SNAP_TO_THRESHOLD {
        Some(closest_y)
    } else {
        None
    };

    (x, y)
}

fn draw_closest_guide(app: &App, draw: &Draw, model: &Model, window: Rect) {
    let (x, y) = find_closest_guides(app, model);

    if let Some(x) = x {
        draw.line()
            .start(Vec2::new(x, window.top()))
            .end(Vec2::new(x, window.bottom()))
            .color(RED);
    }
    if let Some(y) = y {
        draw.line()
            .start(Vec2::new(window.left(), y))
            .end(Vec2::new(window.right(), y))
            .color(RED);
    }
}
