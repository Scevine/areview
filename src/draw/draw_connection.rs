use crate::{Connection, Direction, Exit, Model};
use nannou::prelude::*;

const CONNECTION_LABELS: &'static [&'static str] = &[
    "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "Z", "Y", "Z", "Γ", "Δ", "Θ", "Λ", "Ξ", "Π", "Σ", "Φ", "Ψ", "Ω",
];

pub fn draw_connections(draw: &Draw, model: &Model) {
    let mut endcap_symbol = CONNECTION_LABELS.iter().cycle();
    for connection in &model.connections {
        match connection {
            Connection::TwoWay { from, to, door } => {
                if is_updown_connection(from, to) {
                    let symbol = endcap_symbol.next().unwrap();
                    draw_disconnected_connection(
                        &draw,
                        model,
                        from,
                        ConnectionEndCap::Symbol(symbol),
                    );
                    draw_disconnected_connection(
                        &draw,
                        model,
                        to,
                        ConnectionEndCap::Symbol(symbol),
                    );
                } else {
                    draw_connection(&draw, &model, from, to, false, *door);
                }
            }
            Connection::OneWay { from, to, door } => {
                if is_updown_connection(from, to) {
                    let symbol = endcap_symbol.next().unwrap();
                    draw_disconnected_connection(
                        &draw,
                        model,
                        from,
                        ConnectionEndCap::Symbol(symbol),
                    );
                    draw_disconnected_connection(
                        &draw,
                        model,
                        to,
                        ConnectionEndCap::Symbol(symbol),
                    );
                } else {
                    draw_connection(&draw, &model, from, to, true, *door);
                }
            }
            Connection::External { from, to, .. } => {
                draw_disconnected_connection(&draw, &model, from, ConnectionEndCap::Vnum(&to));
            }
        }
    }
}

fn draw_connection(draw: &Draw, model: &Model, from: &Exit, to: &Exit, one_way: bool, door: bool) {
    let mut from_origin = model.locations[from.index];
    let mut to_origin = model.locations[to.index];
    if let Some(offset) = model.ui.grab_offset {
        if model.selected[from.index] {
            from_origin += offset;
        }
        if model.selected[to.index] {
            to_origin += offset;
        }
    }
    draw.line()
        .stroke_weight(2f32)
        .start(from_origin)
        .end(to_origin);
}

fn draw_disconnected_connection(
    draw: &Draw,
    model: &Model,
    from: &Exit,
    end_cap: ConnectionEndCap,
) {
    let mut start = model.locations[from.index];
    if let Some(offset) = model.ui.grab_offset {
        if model.selected[from.index] {
            start += offset;
        }
    }
    let delta = match from.direction {
        Direction::North => Vec2::new(0f32, model.square_size()),
        Direction::East => Vec2::new(model.square_size(), 0f32),
        Direction::South => Vec2::new(0f32, model.square_size() * -1f32),
        Direction::West => Vec2::new(model.square_size() * -1f32, 0f32),
        Direction::Up => Vec2::default() + model.square_size(),
        Direction::Down => Vec2::default() - model.square_size(),
    };
    let end = start + delta;
    draw.line()
        .stroke_weight(2f32)
        .start(start)
        .end(end)
        .color(BLACK);
    match end_cap {
        ConnectionEndCap::Vnum(vnum) => {
            draw.xy(start + delta + delta * 0.5).text(vnum).color(RED);
        }
        ConnectionEndCap::Symbol(label) => {
            let draw = draw.xy(start + delta);
            draw.ellipse()
                .radius(model.square_size() * 0.25)
                .stroke(BLACK)
                .stroke_weight(2f32)
                .finish();
            draw.text(label).color(BLACK);
        }
    }
}

fn is_updown_connection(left: &Exit, right: &Exit) -> bool {
    match (left.direction, right.direction) {
        (Direction::Up, _) | (Direction::Down, _) | (_, Direction::Up) | (_, Direction::Down) => {
            true
        }
        _ => false,
    }
}

enum ConnectionEndCap<'a> {
    Vnum(&'a str),
    Symbol(&'a str),
}
