use crate::room::{Direction, Room, Vnum};
use fnv::FnvHashMap;
use std::rc::Rc;

pub fn sort_rooms(rooms: Vec<Room>) -> Vec<Vec<Rc<Room>>> {
    let rooms: Vec<_> = rooms.into_iter().map(|r| Rc::new(r)).collect();
    let all_rooms = {
        let mut hash = FnvHashMap::default();
        for room in &rooms {
            hash.insert(room.vnum, room.clone());
        }
        hash
    };

    let mut floors_hash = FnvHashMap::default();
    for room in rooms {
        let floor = {
            let (floor, _) = floors_hash
                .entry(room.vnum)
                .or_insert_with(|| (0i32, room.clone()));
            *floor
        };
        for (dir, dest_vnum) in &room.exits {
            if floors_hash.contains_key(dest_vnum) {
                continue;
            }
            if let Some(dest) = all_rooms.get(dest_vnum).cloned() {
                match dir {
                    Direction::Up => {
                        floors_hash.insert(*dest_vnum, (floor + 1, dest.clone()));
                    }
                    Direction::Down => {
                        floors_hash.insert(*dest_vnum, (floor - 1, dest.clone()));
                    }
                    _ => {
                        floors_hash.insert(*dest_vnum, (floor, dest.clone()));
                    }
                }
            }
        }
    }

    let mut floors = FnvHashMap::default();
    for (_, (floor_num, dest)) in floors_hash {
        let floor = floors.entry(floor_num).or_insert_with(|| vec![]);
        floor.push(dest);
    }

    floors.into_values().collect()
}

fn find_rooms_in_plane(
    room: Option<Rc<Room>>,
    left_to_visit: &mut Vec<Rc<Room>>,
) -> Vec<Vec<Rc<Room>>> {
    let mut this_plane = vec![];

    // Function should not be called with no rooms left to visit
    if left_to_visit.is_empty() {
        return vec![this_plane];
    }

    let mut queue = vec![room.unwrap_or_else(|| left_to_visit.remove(0))];
    let mut queue_for_different_plane = vec![];

    while !queue.is_empty() {
        let room = queue.pop().unwrap();
        this_plane.push(room.clone());

        for (dir, dest) in &room.exits {
            // Find the connected room
            if let Some(dest_room) = left_to_visit.iter().find(|r| r.vnum == *dest).cloned() {
                // Should not be possible unless `left_to_visit` contained duplicate VNUMs
                if this_plane.contains(&dest_room) || queue.contains(&dest_room) {
                    continue;
                }

                match dir {
                    Direction::Up | Direction::Down => {
                        queue_for_different_plane.push(dest_room);
                    }
                    _ => {
                        // If room was queued as an up/down connection for a separate plane,
                        // remove that reference and queue it for this plane instead
                        if let Some(idx) = queue_for_different_plane
                            .iter()
                            .position(|r| r == &dest_room)
                        {
                            queue_for_different_plane.remove(idx);
                        }
                        queue.push(dest_room);
                    }
                }
            }
        }
    }

    // Remove rooms visited on this plane from the to-visit list
    left_to_visit.retain(|r| !this_plane.contains(r));

    std::iter::once(this_plane)
        .chain(
            queue_for_different_plane
                .into_iter()
                .flat_map(|room| find_rooms_in_plane(Some(room), left_to_visit)),
        )
        .collect()

    // TODO: handle the orphaned rooms that are still in left_to_visit
}

#[cfg(test)]
mod test {
    use super::{find_rooms_in_plane, Direction, Rc, Room};

    #[test]
    fn find_rooms_in_plane_groups_nsew_connections() {
        let mut rooms = vec![
            Rc::new(Room {
                vnum: 1000,
                name: "1000".into(),
                exits: [(Direction::North, 1001u32), (Direction::West, 500u32)]
                    .into_iter()
                    .collect(),
            }),
            Rc::new(Room {
                vnum: 1001,
                name: "1001".into(),
                exits: [(Direction::South, 1000u32)].into_iter().collect(),
            }),
        ];

        let original_rooms = vec![rooms.clone()];

        let plane = find_rooms_in_plane(None, &mut rooms);
        assert_eq!(plane, original_rooms);
    }

    #[test]
    fn find_rooms_in_plane_separates_updown_connections() {
        let mut rooms = vec![
            Rc::new(Room {
                vnum: 1000,
                name: "1000".into(),
                exits: [(Direction::North, 1001u32), (Direction::Up, 1002u32)]
                    .into_iter()
                    .collect(),
            }),
            Rc::new(Room {
                vnum: 1001,
                name: "1001".into(),
                exits: [(Direction::South, 1000u32)].into_iter().collect(),
            }),
            Rc::new(Room {
                vnum: 1002,
                name: "1002".into(),
                exits: [(Direction::Down, 1000u32)].into_iter().collect(),
            }),
        ];

        let plane = find_rooms_in_plane(None, &mut rooms);
        assert_eq!(
            plane,
            vec![
                vec![
                    Rc::new(Room {
                        vnum: 1000,
                        name: "1000".into(),
                        exits: [(Direction::North, 1001u32), (Direction::Up, 1002u32)]
                            .into_iter()
                            .collect()
                    }),
                    Rc::new(Room {
                        vnum: 1001,
                        name: "1001".into(),
                        exits: [(Direction::South, 1000u32)].into_iter().collect()
                    }),
                ],
                vec![Rc::new(Room {
                    vnum: 1002,
                    name: "1002".into(),
                    exits: [(Direction::Down, 1000u32)].into_iter().collect(),
                })],
            ]
        );
    }
}
