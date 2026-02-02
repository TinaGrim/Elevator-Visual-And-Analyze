use egui::{self, Options, TextureHandle, TextureId, Vec2};
use std::fmt;
#[derive(Debug)]
pub enum Action_Status {
    STOP,
    IDLE,
    UP,
    DOWN,
}
#[derive(Debug, Clone)]
pub enum Door {
    CLOSE,
    OPEN,
}
// #[derive(Debug)]
pub struct Elevator_Object {
    id: u32,
    Status: Action_Status,
    door: Door,
    // position
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,

    Speed: f32,
    Image: Option<TextureHandle>,
}
impl fmt::Debug for Elevator_Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Elevator_Object")
            .field("Id", &self.id)
            .field("Status", &self.Status)
            .field("Door", &self.door)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("velocity_x", &self.velocity_x)
            .field("velocity_y", &self.velocity_y)
            .field("Speed", &self.Speed)
            .field("Image", &self.Image.is_some()) // Show bool instead
            .finish()
    }
}
impl Elevator_Object {
    pub fn new(id: u32, x: f32, y: f32) -> Self {
        Self {
            id: id,
            Status: Action_Status::IDLE,
            door: Door::CLOSE,
            x: x,
            y: y,
            velocity_x: 0.0,
            velocity_y: 0.0,
            Speed: 0.0,
            Image: None,
        }
    }
    pub fn set_position(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    pub fn set_image(&mut self, image: TextureHandle) {
        self.Image = Some(image);
    }
    pub fn texture_id(&self) -> Option<egui::TextureId> {
        self.Image.as_ref().map(|tex| tex.id())
    }
    pub fn toggle_door(&mut self) -> Door {
        self.door = match self.door {
            Door::CLOSE => Door::OPEN,
            Door::OPEN => Door::CLOSE,
        };
        self.door.clone()
    }
}
