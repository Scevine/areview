use crate::room::{Direction, Room, Vnum};
use fnv::FnvHashMap;
use std::rc::Rc;

pub fn sort_rooms(rooms: Vec<Room>) -> (FnvHashMap<Vnum, Rc<Room>>, Vec<Vec<Rc<Room>>>) {
    let mut rooms: Vec<_> = rooms.into_iter().map(|r| Rc::new(r)).collect();
    let hash: FnvHashMap<Vnum, Rc<Room>> =
        rooms.iter().map(|room| (room.vnum, room.clone())).collect();
    let planes = find_rooms_in_plane(None, &mut rooms);
    (hash, planes)
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

    let mut planes: Vec<_> = std::iter::once(this_plane)
        .chain(
            queue_for_different_plane
                .into_iter()
                .flat_map(|room| find_rooms_in_plane(Some(room), left_to_visit)),
        )
        .collect();

    while !left_to_visit.is_empty() {
        let mut other_planes = find_rooms_in_plane(None, left_to_visit);
        planes.append(&mut other_planes);
    }

    planes.retain(|plane| !plane.is_empty());

    planes
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

        let planes = find_rooms_in_plane(None, &mut rooms);
        assert_eq!(planes, original_rooms);
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

        let planes = find_rooms_in_plane(None, &mut rooms);
        assert_eq!(
            planes,
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

    #[test]
    fn find_rooms_in_plane_includes_orphaned_planes() {
        let mut rooms = vec![
            Rc::new(Room {
                vnum: 1000,
                name: "1000".into(),
                exits: [(Direction::North, 1001u32)].into_iter().collect(),
            }),
            Rc::new(Room {
                vnum: 1001,
                name: "1001".into(),
                exits: [(Direction::South, 1000u32)].into_iter().collect(),
            }),
            Rc::new(Room {
                vnum: 1002,
                name: "1002".into(),
                exits: [].into_iter().collect(),
            }),
        ];

        let planes = find_rooms_in_plane(None, &mut rooms);
        assert_eq!(
            planes,
            vec![
                vec![
                    Rc::new(Room {
                        vnum: 1000,
                        name: "1000".into(),
                        exits: [(Direction::North, 1001u32)].into_iter().collect()
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
                    exits: [].into_iter().collect(),
                })],
            ]
        )
    }
}
