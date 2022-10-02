mod room;

use fnv::FnvHashMap;
use std::error::Error;
use std::path::Path;

use room::{parse_rooms, Direction, Room};

type HashMap<T, V> = FnvHashMap<T, V>;

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No path to area file supplied!");
        std::process::exit(1);
    });
    let rooms = load_area(&path)
        .map_err(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        })
        .unwrap();
    let floors = sort_rooms(rooms);
    println!("{:?}", floors);
    Ok(())
}

fn load_area(path: &dyn AsRef<Path>) -> Result<Vec<Room>, Box<dyn Error>> {
    let file = std::fs::read_to_string(path)?;
    parse_rooms(&file)
}

fn sort_rooms(rooms: Vec<Room>) -> Vec<Vec<Room>> {
    let all_rooms = {
        let mut hash = HashMap::default();
        for room in &rooms {
            hash.insert(room.vnum, room.clone());
        }
        hash
    };

    let mut floors_hash = HashMap::default();
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

    let mut floors = HashMap::default();
    for (_, (floor_num, dest)) in floors_hash {
        let floor = floors.entry(floor_num).or_insert_with(|| vec![]);
        floor.push(dest);
    }

    floors.into_values().collect()
}
