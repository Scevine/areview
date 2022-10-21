use crate::model::{Direction, Door, Room, Vnum};
use std::rc::Rc;

pub enum Connection {
    TwoWay { from: Exit, to: Exit, door: Door },
    OneWay { from: Exit, to: Exit, door: Door },
    External { from: Exit, to: Vnum, door: Door },
}

#[derive(Debug, Eq, PartialEq)]
pub struct Exit {
    pub direction: Direction,
    pub in_room: Vnum,
}

pub fn find_connections(rooms: &[Rc<Room>]) -> Vec<Connection> {
    let mut connections: Vec<Connection> = Vec::new();

    for room in rooms {
        for (&direction, (destination, from_door)) in &room.exits {
            let exit = Exit {
                direction,
                in_room: room.vnum,
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
            let conn = if let Some(dest) = rooms.iter().find(|r| r.vnum == *destination) {
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
                        },
                        door: from_door.or(to_door),
                    }
                } else {
                    Connection::OneWay {
                        from: exit,
                        to: Exit {
                            direction: direction.opposite(),
                            in_room: dest.vnum,
                        },
                        door: *from_door,
                    }
                }
            } else {
                Connection::External {
                    from: exit,
                    to: *destination,
                    door: *from_door,
                }
            };

            connections.push(conn);
        }
    }

    connections
}
