use super::Location;

#[derive(Debug, Clone)]
pub struct Group {
    pub x: f32,
    pub y: f32,
    pub center_x: f32,
    pub center_y: f32,
    pub locations: Vec<Location>,
}

impl Group {
    pub fn new(locations: Vec<Location>) -> Group {
        let (center_x, center_y) = Group::center(&locations);

        Group {
            locations,
            x: 0f32,
            y: 0f32,
            center_x,
            center_y,
        }
    }

    fn center(locations: &[Location]) -> (f32, f32) {
        let mut max_x = 0f32;
        let mut min_x = 0f32;
        let mut max_y = 0f32;
        let mut min_y = 0f32;

        for loc in locations {
            max_x = loc.x.max(max_x);
            min_x = loc.x.min(min_x);
            max_y = loc.y.max(max_y);
            min_y = loc.y.min(min_y);
        }

        ((max_x + min_x) / 2f32, (max_y + min_y) / 2f32)
    }
}
