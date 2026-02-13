use crate::elevator::{Door, ElevatorObject};
use egui::{Align2, Color32, FontFamily, FontId, Rect, Response, Sense, Vec2, Widget};


#[derive(Debug)]
pub struct ElevatorWidget<'a> {
    object: &'a mut ElevatorObject,
    door: bool,
    size: Vec2,
}
impl<'a> ElevatorWidget<'a> {
    pub fn new(object: &'a mut ElevatorObject, size: Vec2) -> Self {
        Self {
            object,
            door: false,
            size,
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
        if response.clicked() {
            match self.object.toggle_door() {
                Door::Close => println!("Elevator Door : CLOSED"),
                Door::Open => println!("Elevator Door : OPENED"),
            }
        }

        if let Some(image) = self.object.texture_id() {
            painter.image(
                image,
                rect,
                Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        } else {
            painter.rect(
                rect,
                5.0,
                Color32::from_rgb(50, 35, 23),
                egui::Stroke::new(2.0, Color32::from_rgb(50, 100, 150)),
            );
        }
        response
    }
}
