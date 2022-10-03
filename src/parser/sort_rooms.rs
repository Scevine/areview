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
