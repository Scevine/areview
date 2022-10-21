use crate::model::{Direction, Door, Room, Vnum};
use crate::parser;
use fnv::FnvHashMap;
use std::rc::Rc;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Connection {
    TwoWay { from: Exit, to: Exit, door: Door },
    OneWay { from: Exit, to: Exit, door: Door },
    External { from: Exit, to: String, door: Door },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Exit {
    pub direction: Direction,
    pub in_room: Vnum,
    pub index: usize,
}

pub fn map_connection(
    c: parser::Connection,
    all_rooms: &FnvHashMap<Vnum, (Rc<Room>, usize)>,
) -> Connection {
    match c {
        parser::Connection::OneWay { from, to, door } => Connection::OneWay {
            from: map_exit(from, &all_rooms),
            to: map_exit(to, &all_rooms),
            door,
        },
        parser::Connection::TwoWay { from, to, door } => Connection::TwoWay {
            from: map_exit(from, &all_rooms),
            to: map_exit(to, &all_rooms),
            door,
        },
        parser::Connection::External { from, to, door } => Connection::External {
            from: map_exit(from, &all_rooms),
            to: to.to_string(),
            door,
        },
    }
}

fn map_exit(parsed: parser::Exit, all_rooms: &FnvHashMap<Vnum, (Rc<Room>, usize)>) -> Exit {
    let parser::Exit { direction, in_room } = parsed;
    Exit {
        direction,
        in_room,
        index: all_rooms.get(&in_room).unwrap().1,
    }
}
