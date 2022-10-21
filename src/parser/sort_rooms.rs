use crate::model::{Direction, Room, Vnum};
use crate::parser::rule::Rule;
use fnv::FnvHashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub x: i32,
    pub y: i32,
    pub room: Rc<Room>,
}

#[derive(Debug, Clone)]
pub struct Rect {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

pub fn sort_rooms(
    rooms: Vec<Rc<Room>>,
    rules: Vec<Rule>,
) -> (FnvHashMap<Vnum, (Rc<Room>, usize)>, Vec<Vec<Location>>) {
    let mut rooms = rooms.clone();
    rooms.sort_by(|a, b| a.vnum.cmp(&b.vnum));
    let by_vnum: FnvHashMap<Vnum, (Rc<Room>, usize)> = rooms
        .iter()
        .cloned()
        .enumerate()
        .map(|(idx, room)| (room.vnum, (room, idx)))
        .collect();

    let planes = find_rooms_in_plane(None, &mut rooms, &rules);
    (by_vnum, planes)
}

fn find_rooms_in_plane(
    location: Option<Location>,
    left_to_visit: &mut Vec<Rc<Room>>,
    rules: &[Rule],
) -> Vec<Vec<Location>> {
    let mut this_plane = vec![];

    // Function should not be called with no rooms left to visit
    if left_to_visit.is_empty() {
        return vec![this_plane];
    }

    let mut queue = std::collections::VecDeque::with_capacity(left_to_visit.len());
    queue.push_back(location.unwrap_or(Location {
        x: 0,
        y: 0,
        room: left_to_visit[0].clone(),
    }));
    let mut queue_for_different_plane = vec![];

    while !queue.is_empty() {
        let location = queue.pop_front().unwrap();
        this_plane.push(location.clone());

        for (dir, (dest, _)) in &location.room.exits {
            // Find the connected room
            if let Some(dest_room) = left_to_visit.iter().find(|r| r.vnum == *dest).cloned() {
                // Should not be possible unless `left_to_visit` contained duplicate VNUMs
                if this_plane.iter().any(|l| l.room == dest_room)
                    || queue.iter().any(|l| l.room == dest_room)
                {
                    continue;
                }

                // If the connection is only one way, AND the destination's matching exit goes to a
                // different room, consider it to be on a different plane
                if !dest_room
                    .exits
                    .values()
                    .any(|(vnum, _)| *vnum == location.room.vnum)
                {
                    queue_for_different_plane.push(dest_room);
                    continue;
                }

                if rules.iter().any(|rule| match rule {
                    Rule::Isolate(vnum) if *vnum == location.room.vnum => true,
                    Rule::Separate(vnum, other) if *vnum == location.room.vnum && other == dest => {
                        true
                    }
                    Rule::Separate(other, vnum) if *vnum == location.room.vnum && other == dest => {
                        true
                    }
                    _ => false,
                }) {
                    queue_for_different_plane.push(dest_room);
                    continue;
                }

                match dir {
                    Direction::Up | Direction::Down => {
                        queue_for_different_plane.push(dest_room);
                    }
                    _ => {
                        let (x, y) = match dir {
                            Direction::North => (0, 1),
                            Direction::East => (1, 0),
                            Direction::South => (0, -1),
                            Direction::West => (-1, 0),
                            _ => unreachable!(),
                        };
                        // If room was queued as an up/down connection for a separate plane,
                        // remove that reference and queue it for this plane instead
                        if let Some(idx) = queue_for_different_plane
                            .iter()
                            .position(|r| r == &dest_room)
                        {
                            queue_for_different_plane.remove(idx);
                        }
                        queue.push_back(Location {
                            x: location.x + x,
                            y: location.y + y,
                            room: dest_room,
                        });
                    }
                }
            }
        }
    }

    // Remove rooms visited on this plane from the to-visit list
    left_to_visit.retain(|r| !this_plane.iter().any(|l| &l.room == r));

    let mut planes = vec![this_plane];
    for room in queue_for_different_plane.into_iter() {
        let mut more_planes =
            find_rooms_in_plane(Some(Location { room, x: 0, y: 0 }), left_to_visit, rules);
        planes.append(&mut more_planes);
    }

    while !left_to_visit.is_empty() {
        let mut more_planes = find_rooms_in_plane(None, left_to_visit, rules);
        planes.append(&mut more_planes);
    }

    planes.retain(|plane| !plane.is_empty());

    planes
}

#[cfg(test)]
mod test {
    use super::{find_rooms_in_plane, Direction, Location, Rc, Room, Vnum};
    use crate::model::{Door, Sector};

    fn make_room(vnum: Vnum, exits: &[(Direction, (u32, Door))]) -> Rc<Room> {
        Rc::new(Room {
            vnum,
            name: vnum.to_string(),
            string_vnum: vnum.to_string(),
            sector: Sector::Inside,
            exits: exits.iter().copied().collect(),
        })
    }

    fn rooms_from_locations(groups: Vec<Vec<Location>>) -> Vec<Vec<Rc<Room>>> {
        groups
            .into_iter()
            .map(|group| group.into_iter().map(|loc| loc.room).collect::<Vec<_>>())
            .collect()
    }

    #[test]
    fn find_rooms_in_plane_groups_nsew_connections() {
        let mut rooms = vec![
            make_room(
                1000,
                &[
                    (Direction::North, (1001u32, Door::None)),
                    (Direction::West, (500u32, Door::None)),
                ],
            ),
            make_room(1001, &[(Direction::South, (1000u32, Door::None))]),
        ];

        let original_rooms = vec![rooms.clone()];

        let planes = find_rooms_in_plane(None, &mut rooms, &[]);
        let planes = rooms_from_locations(planes);
        assert_eq!(planes, original_rooms);
    }

    #[test]
    fn find_rooms_in_plane_separates_updown_connections() {
        let mut rooms = vec![
            make_room(
                1000,
                &[
                    (Direction::North, (1001u32, Door::None)),
                    (Direction::Up, (1002u32, Door::None)),
                ],
            ),
            make_room(1001, &[(Direction::South, (1000u32, Door::None))]),
            make_room(1002, &[(Direction::Down, (1000u32, Door::None))]),
        ];

        let planes = find_rooms_in_plane(None, &mut rooms, &[]);
        let planes = rooms_from_locations(planes);
        assert_eq!(
            planes,
            vec![
                vec![
                    make_room(
                        1000,
                        &[
                            (Direction::North, (1001u32, Door::None)),
                            (Direction::Up, (1002u32, Door::None))
                        ]
                    ),
                    make_room(1001, &[(Direction::South, (1000u32, Door::None))]),
                ],
                vec![make_room(1002, &[(Direction::Down, (1000u32, Door::None))])],
            ]
        );
    }

    #[test]
    fn find_rooms_in_plane_includes_orphaned_planes() {
        let mut rooms = vec![
            make_room(1000, &[(Direction::North, (1001u32, Door::None))]),
            make_room(1001, &[(Direction::South, (1000u32, Door::None))]),
            make_room(1002, &[]),
        ];

        let planes = find_rooms_in_plane(None, &mut rooms, &[]);
        let planes = rooms_from_locations(planes);
        assert_eq!(
            planes,
            vec![
                vec![
                    make_room(1000, &[(Direction::North, (1001u32, Door::None))]),
                    make_room(1001, &[(Direction::South, (1000u32, Door::None))]),
                ],
                vec![make_room(1002, &[])],
            ]
        )
    }
}
