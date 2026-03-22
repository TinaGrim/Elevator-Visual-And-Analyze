use crate::Elevator;
use egui::{
    vec2, Color32, ColorImage, Image, ImageData, Sense, TextureHandle, TextureOptions, Vec2,
};
use rand::Rng;
#[derive(Clone)]
pub struct HumanObject {
    pub name: String,

    // position
    x: f32,
    y: f32,

    velocity_x: f32,
    velocity_y: f32,

    destination: f32,
    floor: String,
    is_request: bool,
    pub has_requested: bool,
    request_floor: Option<String>,
    pub reach_elevator: bool,

    speed: f32,
    image: Option<TextureHandle>,
}
impl std::fmt::Debug for HumanObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Human Object")
            .field("name", &self.name)
            .field("x", &self.x)
            .field("y", &self.y)
            .field("velocity_x", &self.velocity_x)
            .field("velocity_y", &self.velocity_y)
            .field("human destination", &self.destination)
            .field("is request", &self.is_request)
            .field("has requested", &self.has_requested)
            .field("request floor", &self.request_floor.is_none())
            .field("speed", &self.speed)
            .field("image", &self.image.is_some())
            .finish()
    }
}
impl HumanObject {
    pub fn new(name: String, floor: String, x: f32, y: f32) -> Self {
        Self {
            name,
            x,
            y,

            velocity_x: 0.0,
            velocity_y: 0.0,
            destination: rand::thread_rng().gen_range(350.0..400.0),
            is_request: false,
            has_requested: false,
            request_floor: None,
            reach_elevator: false,
            floor,
            speed: 5.0,
            image: None,
        }
    }

    pub fn update(&mut self) {
        if self.x < self.destination {
            self.x += 1.0;
        }
    }
    pub fn floor(&self) -> &str {
        &self.floor
    }
    pub fn request_floor(&mut self) -> Option<String> {
        if self.is_request && !self.has_requested {
            self.has_requested = true;
            Some(self.floor.clone())
        } else {
            None
        }
    }
    pub fn enter_elevator(&mut self, position: (f32, f32)) {
        self.destination = position.0 + rand::thread_rng().gen_range(45.0..50.0);
    }
    pub fn get_current_floor(&self) -> String {
        self.floor.clone()
    }
    pub fn set_image(&mut self, image: TextureHandle) {
        self.image = Some(image);
    }
    pub fn reach_destination(&mut self) -> bool {
        if self.x >= self.destination {
            self.is_request = true;
            true
        } else {
            false
        }
    }
    pub fn texture_id(&self) -> egui::TextureId {
        self.image.clone().unwrap().id()
    }
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    pub fn set_position(&mut self, x: f32, y: f32, available: egui::Rect) {
        (self.x, self.y) = (available.min.x + x, available.min.y + y);
    }
}
