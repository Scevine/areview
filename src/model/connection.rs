use crate::model::{Direction, Door, Room, Vnum};
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
    in_room: Vnum,
    pub index: usize,
}

pub fn find_connections(
    rooms: &FnvHashMap<Vnum, Rc<Room>>,
    indexes_by_vnum: FnvHashMap<Vnum, usize>,
) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();

    for room in rooms.values() {
        for (&direction, (destination, from_door)) in &room.exits {
            let exit = Exit {
                direction,
                in_room: room.vnum,
                index: *indexes_by_vnum
                    .get(&room.vnum)
                    .expect("Room vnum wasn't included in vnum->index map"),
            };
            if connections.iter().any(|conn| match conn {
                Connection::TwoWay { from, .. } if *from == exit => true,
                Connection::TwoWay { to, .. } if *to == exit => true,
                Connection::OneWay { from, .. } if *from == exit => true,
                Connection::External { from, .. } if *from == exit => true,
                _ => false,
            }) {
                continue;
            }
            let conn = if let Some(dest) = rooms.get(destination) {
                let matching_exit = dest
                    .exits
                    .iter()
                    .find(|(dir, (vnum, _))| *vnum == room.vnum && **dir == direction.opposite());
                let matching_exit_in_another_dir =
                    dest.exits.iter().find(|(_, (vnum, _))| *vnum == room.vnum);

                if let Some((&dir, (_, to_door))) = matching_exit.or(matching_exit_in_another_dir) {
                    Connection::TwoWay {
                        from: exit,
                        to: Exit {
                            direction: dir,
                            in_room: dest.vnum,
                            index: *indexes_by_vnum
                                .get(&dest.vnum)
                                .expect("Room vnum wasn't included in vnum->index map"),
                        },
                        door: from_door.or(to_door),
                    }
                } else {
                    Connection::OneWay {
                        from: exit,
                        to: Exit {
                            direction: direction.opposite(),
                            in_room: dest.vnum,
                            index: *indexes_by_vnum
                                .get(&dest.vnum)
                                .expect("Room vnum wasn't invluced in vnum-index map"),
                        },
                        door: *from_door,
                    }
                }
            } else {
                Connection::External {
                    from: exit,
                    to: destination.to_string(),
                    door: *from_door,
                }
            };

            connections.push(conn);
        }
    }

    connections
}
