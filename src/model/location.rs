use crate::room::Room;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Location {
    pub x: f32,
    pub y: f32,
    pub room: Rc<Room>,
}
