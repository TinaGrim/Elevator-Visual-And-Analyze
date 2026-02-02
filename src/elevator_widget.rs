use crate::elevator::{Door, Elevator_Object};
use egui::{Align2, Color32, FontFamily, FontId, Rect, Response, Sense, Vec2, Widget};

enum Action_Status {
    STOP,
    IDLE,
    UP,
    DOWN,
}

#[derive(Debug)]
pub struct Elevator_Widget<'a> {
    Object: &'a mut Elevator_Object,
    Door: bool,
    size: Vec2,
}
impl<'a> Elevator_Widget<'a> {
    pub fn new(object: &'a mut Elevator_Object, size: Vec2) -> Self {
        Self {
            Object: object,
            Door: false,
            size: size,
        }
    }
}
impl<'a> Widget for Elevator_Widget<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        let (x, y) = self.Object.get_position();
        let position = egui::Pos2::new(x, y);

        let rect = egui::Rect::from_min_size(position, self.size);

        let response = ui.allocate_rect(rect, Sense::click_and_drag());
        let painter = ui.painter();
        if response.clicked() {
            match self.Object.toggle_door(){
                Door::CLOSE => println!("Elevator Door : CLOSED"),
                Door::OPEN => println!("Elevator Door : OPENED"),
            }
        }

        if let Some(image) = self.Object.texture_id() {
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
