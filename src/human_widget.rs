use crate::human::HumanObject;

use egui::{
    Color32, ColorImage, Image, ImageData, Response, Sense, TextureHandle, TextureOptions, Vec2,
    Widget, vec2,
};

#[derive(Debug)]
pub struct HumanWidget<'a> {
    object: &'a HumanObject,
    walk: bool,
    size: Vec2,
}
impl<'a> HumanWidget<'a> {
    fn new(object: &'a HumanObject, size: Vec2) -> Self {
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
        let rect = egui::Rect::from_min_size(position, self.size);
        let response = ui.allocate_rect(rect, Sense::click_and_drag());

        response
    }
}
