use egui::{self, Options, TextureHandle, TextureId, Vec2};
use std::fmt;
#[derive(Debug)]
pub enum ActionStatus {
    Stop,
    Idle,
    Up,
    Down,
}
#[derive(Debug, Clone)]
pub enum Door {
    Close,
    Open,
}
// #[derive(Debug)]
pub struct ElevatorObject {
    id: u32,
    status: ActionStatus,
    door: Door,

    // position
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    destination: f32,

    speed: f32,
    image: Option<TextureHandle>,
}
impl fmt::Debug for ElevatorObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Elevator_Object")
            .field("Id", &self.id)
            .field("Status", &self.status)
            .field("Door", &self.door)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("velocity_x", &self.velocity_x)
            .field("velocity_y", &self.velocity_y)
            .field("Speed", &self.speed)
            .field("Image", &self.image.is_some()) // Show bool instead
            .finish()
    }
}
impl ElevatorObject {
    pub fn new(id: u32, x: f32, y: f32) -> Self {
        Self {
            id,
            status: ActionStatus::Idle,
            door: Door::Close,
            x,
            y,
            velocity_x: 0.0,
            velocity_y: 0.0,
            destination: 0.0,

            speed: 0.0,
            image: None,
        }
    }
    pub fn update(&mut self) {
        if self.y < self.destination {
            self.y += 1.0;
        } else if self.y > self.destination {
            self.y -= 1.0;
        }
        // println!("Elevator {} position: ({}, {}) destination: {}", self.id, self.x, self.y, self.destination);
    }
    pub fn set_position(&mut self, x: f32, y: f32, available: egui::Rect) {
        self.x = available.min.x + x;
        self.y = available.min.y + y;
    }
    pub fn set_destination(&mut self, y: f32, available: egui::Rect) {
        self.destination = available.min.y.round() + y;
    }
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    pub fn set_image(&mut self, image: TextureHandle) {
        self.image = Some(image);
    }
    pub fn texture_id(&self) -> egui::TextureId {
        self.image.clone().unwrap().id()
    }
    pub fn toggle_door(&mut self) -> Door {
        self.door = match self.door {
            Door::Close => Door::Open,
            Door::Open => Door::Close,
        };
        self.door.clone()
    }
}
