use crate::room::Door;
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
                    draw_disconnected_connection(draw, model, from, to, symbol);
                } else {
                    draw_connection(&draw, &model, from, to, false, *door);
                }
            }
            Connection::OneWay { from, to, door } => {
                if is_updown_connection(from, to) {
                    let symbol = endcap_symbol.next().unwrap();
                    draw_disconnected_connection(draw, model, from, to, symbol);
                } else {
                    draw_connection(&draw, &model, from, to, true, *door);
                }
            }
            Connection::External { from, to, .. } => {
                draw_external_connection(&draw, &model, from, &to);
            }
        }
    }
}

fn draw_connection(draw: &Draw, model: &Model, from: &Exit, to: &Exit, one_way: bool, door: Door) {
    let (p1, p2) = find_exit(model, from, Lean::None);
    let (p4, p3) = find_exit(model, to, Lean::None);
    draw.polyline()
        .weight(2f32)
        .join_round()
        .points(vec![p1, p2, p3, p4]);
    if let Door::Closed = door {
        let middle_of_connection = (p3 + p2) * 0.5;
        draw.xy(middle_of_connection)
            .ellipse()
            .radius(5f32)
            .color(BLACK)
            .finish();
    }
}

fn draw_external_connection(draw: &Draw, model: &Model, exit: &Exit, text: &str) {
    let (p1, p2) = find_exit(model, exit, Lean::None);
    let delta = p2 - p1;
    draw.line().stroke_weight(2f32).start(p1).end(p2);
    draw.xy(p2 + delta * 0.5).text(text).color(RED);
}

fn draw_disconnected_connection(draw: &Draw, model: &Model, from: &Exit, to: &Exit, label: &str) {
    let x1 = location_of(model, from.index).x;
    let x2 = location_of(model, to.index).x;

    let (p1, p2) = find_exit(model, from, if x1 < x2 { Lean::Right } else { Lean::Left });
    draw.line().stroke_weight(2f32).start(p1).end(p2);
    draw.xy(p2)
        .ellipse()
        .radius(model.square_size() * 0.25)
        .stroke(BLACK)
        .stroke_weight(2f32)
        .finish();
    draw.xy(p2).text(label).color(BLACK);

    let (p3, p4) = find_exit(model, to, if x1 < x2 { Lean::Left } else { Lean::Right });
    draw.line().stroke_weight(2f32).start(p3).end(p4);
    draw.xy(p4)
        .ellipse()
        .radius(model.square_size() * 0.25)
        .stroke(BLACK)
        .stroke_weight(2f32)
        .finish();
    draw.xy(p4).text(label).color(BLACK);
}

enum Lean {
    None,
    Left,
    Right,
}

fn find_exit(model: &Model, exit: &Exit, lean: Lean) -> (Vec2, Vec2) {
    let start = location_of(model, exit.index);
    let delta = match exit.direction {
        Direction::North => Vec2::new(0f32, model.square_size()),
        Direction::East => Vec2::new(model.square_size(), 0f32),
        Direction::South => Vec2::new(0f32, model.square_size() * -1f32),
        Direction::West => Vec2::new(model.square_size() * -1f32, 0f32),
        Direction::Up => match lean {
            Lean::None | Lean::Right => Vec2::default() + model.square_size(),
            Lean::Left => Vec2::new(model.square_size() * -1f32, model.square_size()),
        },
        Direction::Down => match lean {
            Lean::None | Lean::Left => Vec2::default() - model.square_size(),
            Lean::Right => Vec2::new(model.square_size(), model.square_size() * -1f32),
        },
    };
    let end = start + delta;
    (start, end)
}

fn location_of(model: &Model, index: usize) -> Vec2 {
    if model.selected[index] {
        model.locations[index] + model.ui.grab_offset.unwrap_or_default()
    } else {
        model.locations[index]
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
