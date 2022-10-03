mod parser;
mod room;

use parser::load_area;

fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("No path to area file supplied!");
        std::process::exit(1);
    });
    let rooms_by_floor = match load_area(&path) {
        Ok(rooms) => rooms,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(1);
        }
    };
    println!("{:?}", rooms_by_floor);
}
