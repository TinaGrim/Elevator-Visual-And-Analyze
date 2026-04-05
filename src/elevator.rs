use egui::{self, TextureHandle};
use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub enum ActionStatus {
    Stop,
    Idle,
    Up,
    Down,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Door {
    Close,
    Open,
}
// #[derive(Debug)]
pub struct ElevatorObject {
    id: u32,
    pub status: ActionStatus,
    pub door: Door,

    // position
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    destination: f32,
    floor: String,

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
            .field("Floor", &self.floor)
            .field("Speed", &self.speed)
            .field("Image", &self.image.is_some()) // Show bool instead
            .finish()
    }
}
impl ElevatorObject {
    pub fn new(id: u32, floor: String, x: f32, y: f32) -> Self {
        Self {
            id,
            status: ActionStatus::Idle,
            door: Door::Close,
            x,
            y,
            velocity_x: 0.0,
            velocity_y: 0.0,
            floor,
            destination: 0.0,

            speed: 0.0,
            image: None,
        }
    }
    pub fn update(&mut self) {
        if self.y < self.destination {
            self.status = ActionStatus::Down;
            self.y += 1.0;
        } else if self.y > self.destination {
            self.status = ActionStatus::Up;
            self.y -= 1.0;
        } else {
            if self.status == ActionStatus::Up || self.status == ActionStatus::Down {
                self.status = ActionStatus::Stop;
            }
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
    pub fn set_floor(&mut self, floor: String) {
        self.floor = floor
    }
    pub fn get_current_floor(&self) -> String {
        self.floor.clone()
    }
    pub fn reach_destination(&mut self) -> bool {
        self.y == self.destination
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
    pub fn door_status(&self) -> String {
        match self.door {
            Door::Close => "CLOSE".to_string(),
            Door::Open => "OPEN".to_string(),
        }
    }
    pub fn toggle_door(&mut self) {
        match self.door {
            Door::Close => self.door = Door::Open,
            Door::Open => self.door = Door::Close,
        }
    }
}
