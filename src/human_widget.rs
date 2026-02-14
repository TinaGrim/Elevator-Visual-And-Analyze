use crate::human::HumanObject;

use egui::{
    Color32, ColorImage, Image, ImageData, Response, Sense, TextureHandle, TextureOptions, Vec2,
    Widget, vec2,
};

#[derive(Debug)]
pub struct HumanWidget<'a> {
    object: &'a mut HumanObject,
    walk: bool,
    size: Vec2,
}
impl<'a> HumanWidget<'a> {
    pub fn new(object: &'a mut HumanObject, size: Vec2) -> Self {
        Self {
            object,
            walk: false,
            size,
        }
    }
}
impl<'a> Widget for HumanWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let (x, y) = self.object.get_position();
        let position = egui::pos2(x, y);
        let rect = egui::Rect::from_center_size(position, self.size);
        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        if let Some(textureid) = self.object.texture_id() {
            ui.painter().image(
                textureid,
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        };

        response
    }
}
