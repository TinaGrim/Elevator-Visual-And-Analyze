use crate::elevator::{ActionStatus, ElevatorObject};
use egui::{Color32, Rect, Response, Sense, TextureHandle, Vec2, Widget};

pub struct ElevatorWidget<'a> {
    object: &'a mut ElevatorObject,
    size: Vec2,
    green_light: &'a TextureHandle,
    red_light: &'a TextureHandle,
}
impl<'a> ElevatorWidget<'a> {
    pub fn new(
        object: &'a mut ElevatorObject,
        size: Vec2,
        green_light: &'a TextureHandle,
        red_light: &'a TextureHandle,
    ) -> Self {
        Self {
            object,
            size,
            green_light,
            red_light,
        }
    }
    fn get_texture_for_status(&self) -> egui::TextureId {
        match self.object.status {
            ActionStatus::Up => self.green_light.id(),
            ActionStatus::Down => self.red_light.id(),
            ActionStatus::Stop | ActionStatus::Idle => self.object.texture_id(),
        }
    }
}
impl<'a> Widget for ElevatorWidget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let (x, y) = self.object.get_position();
        let position = egui::Pos2::new(x, y);

        let rect = egui::Rect::from_min_size(position, self.size);

        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter();
        if response.clicked() {};

        let image = self.get_texture_for_status();
        painter.image(
            image,
            rect,
            Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::WHITE,
        );
        response
    }
}
