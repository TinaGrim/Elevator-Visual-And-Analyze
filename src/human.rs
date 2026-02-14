use egui::{
    Color32, ColorImage, Image, ImageData, Sense, TextureHandle, TextureOptions, Vec2, vec2,
};

pub struct HumanObject {
    name: String,
    // position
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,

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
            .field("speed", &self.speed)
            .field("image", &self.image.is_some())
            .finish()
    }
}
impl HumanObject {
    pub fn new(name: String, x: f32, y: f32) -> Self {
        Self {
            name,
            x,
            y,
            velocity_x: 0.0,
            velocity_y: 0.0,
            speed: 5.0,
            image: None,
        }
    }
    pub fn set_image(&mut self, image: TextureHandle) {
        self.image = Some(image);
    }
    pub fn texture_id(&self) -> Option<egui::TextureId> {
        self.image.as_ref().map(|tex| tex.id())
    }
    pub fn get_position(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    pub fn set_position(&mut self, x: f32, y: f32, available: egui::Rect) {
        (self.x, self.y) = (available.min.x + x, available.min.y + y);
    }
}
