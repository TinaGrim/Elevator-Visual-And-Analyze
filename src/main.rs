mod elevator;
mod elevator_widget;
use eframe::egui;
use egui::{
    Color32, ColorImage, Image, ImageData, Sense, TextureHandle, TextureOptions, Vec2, vec2,
};

use crate::{elevator::Elevator_Object, elevator_widget::Elevator_Widget};
use egui_extras::install_image_loaders;

struct Elevator {
    run: bool,
    image_loaded: bool,
    texture_handle: Option<TextureHandle>,
    elevator: Elevator_Object,
    current_floor: usize,
    target_floor: usize,
    elevator_y_position: f32,
    floor_positions: Vec<f32>,
}

const FLOOR_LABELS: [&str; 4] = ["G", "1", "2", "3"];

impl Default for Elevator {
    fn default() -> Self {
        Self {
            run: false,
            image_loaded: false,
            texture_handle: None,
            elevator: Elevator_Object::new(1, 0.0, 0.0),
            current_floor: 0,
            target_floor: 0,
            elevator_y_position: 0.0,
            floor_positions: vec![0.0, 200.0, 400.0, 600.0], // Ground, 1, 2, 3
        }
    }
}
impl std::fmt::Debug for Elevator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Elevator")
            .field("run", &self.run)
            .field("image_loaded", &self.image_loaded)
            .field("texture_handle", &self.texture_handle.is_some())
            .field("elevator object", &self.elevator)
            .finish()
    }
}
impl eframe::App for Elevator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !self.image_loaded {
            let image_bytes = include_bytes!("elevator.png");
            let color_image = egui_extras::image::load_image_bytes(image_bytes)
                .expect("Failed to load elevator image");
            self.texture_handle = Some(ctx.load_texture(
                "elevator_texture",
                color_image,
                egui::TextureOptions::default(),
            ));
            self.image_loaded = true;

            if let Some(texture) = &self.texture_handle {
                self.elevator.set_image(texture.clone());
            }
        }

        // Elevator simulation logic
        if self.run {
            let current_pos = self.elevator_y_position;
            let target_pos = self.floor_positions[self.target_floor];
            
            if (current_pos - target_pos).abs() > 2.0 {
                // Move elevator towards target floor
                let speed = 2.0;
                if current_pos < target_pos {
                    self.elevator_y_position += speed;
                } else {
                    self.elevator_y_position -= speed;
                }
            } else {
                // Reached target floor, pick a new random target
                self.current_floor = self.target_floor;
                self.elevator_y_position = target_pos;
                self.target_floor = rand::random::<usize>() % 4;
            }
        }

        egui::TopBottomPanel::top("⚙️ Elevator - Visual Analyze").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚙️ Elevator - Visual And Analyze");
                ui.separator();
                if ui.checkbox(&mut self.run, "🕹️ Start").changed() {
                    if self.run {
                        // Start simulation - pick a random target floor
                        self.target_floor = rand::random::<usize>() % 4;
                    }
                }
                ui.label(format!("Floor: {} → {}", 
                    FLOOR_LABELS[self.current_floor],
                    FLOOR_LABELS[self.target_floor]));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("🏭 Interactive Elevator Simulation");
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let available_rect = ui.available_rect_before_wrap();

                ui.allocate_painter(available_rect.size(), Sense::click_and_drag());

                let grid_size = 200.0;

                draw_grid_lines(ui, available_rect, grid_size);
                draw_floors(ui, available_rect, grid_size);

                let y = available_rect.min.y + self.elevator_y_position;
                self.elevator.set_position(600.0, y);
                let widget = Elevator_Widget::new(&mut self.elevator, Vec2::new(150.0, 195.0));
                ui.add(widget);
            });
        });
        ctx.request_repaint();
    }
}
fn draw_grid_lines(ui: &mut egui::Ui, available_rect: egui::Rect, grid_size: f32) {
    let mut y: f32 = grid_size;
    while y <= available_rect.height() {
        ui.painter().line_segment(
            [
                egui::pos2(available_rect.min.x + 0.0, available_rect.min.y + y),
                egui::pos2(
                    available_rect.min.x + ui.available_width(),
                    available_rect.min.y + y,
                ),
            ],
            egui::Stroke::new(1.0, egui::Color32::GRAY),
        );
        y += grid_size;
    }
}
fn draw_floors(ui: &mut egui::Ui, available_rect: egui::Rect, grid_size: f32) {
    let floors = ["G", "1", "2", "3"];
    let mut start_floor: f32 = available_rect.height() - (grid_size / 2.0);

    for floor in &floors {
        let position = egui::pos2(
            available_rect.min.x + 100.0,
            available_rect.min.y + start_floor,
        );
        let rect = egui::Rect::from_min_size(position, egui::vec2(50.0, 50.0));
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.painter().text(
                //position,
                rect.center(),
                egui::Align2::CENTER_CENTER,
                format!("Floor {}", floor),
                egui::FontId::new(30.0, egui::FontFamily::Monospace),
                egui::Color32::GRAY,
            );
        });
        start_floor -= grid_size;
    }
}
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    println!("Elevator - Visual and Analyze\n");
    eframe::run_native(
        "Elevator - Visual and Analyze\n",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Box::new(Elevator::default())
        }),
    )
}
