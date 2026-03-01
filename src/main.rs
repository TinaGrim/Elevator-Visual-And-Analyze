mod elevator;
mod elevator_widget;

mod human;
mod human_widget;

use core::f32;
use std::time;

use eframe::egui;
use egui::{
    Color32, ColorImage, Image, ImageData, Sense, TextureHandle, TextureOptions, Vec2, pos2, vec2
};
use rand::{Rng, random};
use chrono::{Local, Duration};

use crate::{elevator::ElevatorObject, elevator_widget::ElevatorWidget};
use crate::{human::HumanObject, human_widget::HumanWidget};

use egui_extras::install_image_loaders;

struct Elevator {
    run: bool,
    image_loaded: bool,
    elevator_texture_handle: Option<TextureHandle>,
    human_texture_handle: Option<TextureHandle>,
    elevator: ElevatorObject,
    elevator_rect: Vec2,
    human: Vec<HumanObject>,
    human_rect: Vec<Vec2>,
    human_obstacle: time::Instant,
    floors: Vec<String>,
    button_texture_handle: Vec<TextureHandle>,
}

impl Default for Elevator {
    fn default() -> Self {
        Self {
            run: true,
            image_loaded: false,
            elevator_texture_handle: None,
            human_texture_handle: None,
            elevator: ElevatorObject::new(1, 0.0, 0.0),
            elevator_rect: Vec2::new(150.0, 195.0),
            human: vec![HumanObject::new("Tina".to_string(), "2".to_string(), 0.0, 0.0)],
            human_rect: vec![Vec2::new(130.0, 150.0)],
            human_obstacle: time::Instant::now(),
            floors: vec![
                "G".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ],
            button_texture_handle: Vec::new(),
        }
    }
}
impl std::fmt::Debug for Elevator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Elevator")
            .field("run", &self.run)
            .field("image_loaded", &self.image_loaded)
            .field("texture_handle", &self.elevator_texture_handle.is_some())
            .field("Human_hanle", &self.human_texture_handle.is_some())
            .field("elevator object", &self.elevator)
            .field("elevator rect", &self.elevator_rect)
            .field("button_handle", &self.button_texture_handle.is_empty())
            .finish()
    }
}
impl eframe::App for Elevator {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("⚙️ Elevator - Visualization Analysis").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚙️ Elevator - Visualization And Analysis");
                ui.separator();
                ui.checkbox(&mut self.run, "🕹️ Start");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("🏭 Minimal Working...");
                });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                let available_rect = ui.available_rect_before_wrap();
                if !self.image_loaded {
                    let elevator_image_byes = include_bytes!("elevator.png");
                    let human_image_bytes = include_bytes!("human.png");
                    let normal_button = include_bytes!("normalbutton.png");
                    let red_button = include_bytes!("redbutton.png");
                    let green_button = include_bytes!("greenbutton.png");
                    let both_button = include_bytes!("bothbutton.png");



                    let elevator_color_image = egui_extras::image::load_image_bytes(elevator_image_byes)
                        .expect("Failed to load elevator image");
                    let human_color_image = egui_extras::image::load_image_bytes(human_image_bytes)
                        .expect("Failed to load human image");
                    let normal_button_color_image = egui_extras::image::load_image_bytes(normal_button)
                        .expect("Failed to load normal button image");
                    let red_button_color_image = egui_extras::image::load_image_bytes(red_button)
                        .expect("Failed to load red button image");
                    let green_button_color_image = egui_extras::image::load_image_bytes(green_button)
                        .expect("Failed to load green button image");
                    let both_button_color_image = egui_extras::image::load_image_bytes(both_button)
                        .expect("Failed to load both button image");


                    self.elevator_texture_handle = Some(ctx.load_texture(
                        "elevator_texture",
                        elevator_color_image,
                        egui::TextureOptions::default(),
                    ));
                    self.human_texture_handle = Some(ctx.load_texture(
                        "human_texture", 
                        human_color_image, 
                        egui::TextureOptions::default()
                    ));
                    self.button_texture_handle.push(ctx.load_texture(
                        "normal_button_texture",
                        normal_button_color_image,
                        egui::TextureOptions::default(),
                    ));
                    self.button_texture_handle.push(ctx.load_texture(
                        "red_button_texture",
                        red_button_color_image,
                        egui::TextureOptions::default(),
                    ));
                    self.button_texture_handle.push(ctx.load_texture(
                        "green_button_texture",
                        green_button_color_image,
                        egui::TextureOptions::default(),
                    ));
                    self.button_texture_handle.push(ctx.load_texture(
                        "both_button_texture",
                        both_button_color_image,
                        egui::TextureOptions::default()
                    ));

                    if let Some(texture) = &self.elevator_texture_handle {
                        self.elevator.set_image(texture.clone());
                    }
                    if let Some(texture) = &self.human_texture_handle {
                        self.human[0].set_image(texture.clone());
                    }

                    self.image_loaded = true;

                    at_floor_person(&mut self.human[0], available_rect);
                    at_floor_elevator(&mut self.elevator, "G", available_rect, 500.0);
                    at_floor_elevator_destination(&mut self.elevator, "G", available_rect);
                }

                // Make those frame eaiser
                ui.allocate_painter(available_rect.size(), Sense::click_and_drag());

                let grid_size = 200.0;
                let mut button_floors = vec![self.button_texture_handle[0].clone(); 4];

                draw_grid_lines(ui, available_rect, grid_size);
                draw_floors(ui, self.floors.clone(), available_rect, grid_size);
                draw_buttons(ui, button_floors, available_rect, grid_size);
                ui.input(|input| {
                    for event in &input.events {
                        if let egui::Event::Key {
                            key,
                            pressed: true,
                            repeat: false,
                            ..
                        } = event
                        {
                            match key {
                                egui::Key::Num1 => {
                                    at_floor_elevator_destination(&mut self.elevator, "1", available_rect);
                                    println!("Elevator set floor -> 1\n");
                                }
                                egui::Key::Num2 => {
                                    at_floor_elevator_destination(&mut self.elevator, "2", available_rect);
                                    println!("Elevator set floor -> 2\n");
                                }
                                egui::Key::Num3 => {
                                    at_floor_elevator_destination(&mut self.elevator, "3", available_rect);
                                    println!("Elevator set floor -> 3\n");
                                }
                                egui::Key::G => {
                                    at_floor_elevator_destination(&mut self.elevator, "G", available_rect);
                                    println!("Elevator set floor -> G\n");
                                }

                                _ => println!("Usage: please type the floor for destination. e.p(1, 2, 3, G) \n"),
                            }
                        }
                    }
                });


                if self.human_obstacle.elapsed() >= std::time::Duration::from_secs_f32(5.0) && self.human.len() < 6 {
                    
                    let floor = self.floors[rand::thread_rng().gen_range(0..self.floors.len())].clone();
                    let name = names::Generator::default().next().unwrap();
                    println!("Human created name: {}", name);
                    let mut human = HumanObject::new(name, floor , 0.0, 0.0);
                    at_floor_person(&mut human, available_rect);
                    if let Some(texture) = &self.human_texture_handle {
                        human.set_image(texture.clone());
                    }
                    self.human.push(human);
                    self.human_rect.push(Vec2::new(130.0, 150.0));
                    self.human_obstacle = std::time::Instant::now();

                }
                self.elevator.update();
                for human in &mut self.human {
                    human.update();
                }
                
                

                let elevator = ElevatorWidget::new(&mut self.elevator, self.elevator_rect);
                for (human, &human_rect) in self.human.iter_mut().zip(&self.human_rect) {
                    
                    let human = HumanWidget::new(human, human_rect);
                    ui.add(human);
                }
                ui.add(elevator);
            });
        });

        ctx.request_repaint();
    }
}
fn at_floor_elevator(elevator: &mut ElevatorObject, floor: &str, available_rect: egui::Rect, x: f32) {
    match floor {
        "3" => elevator.set_position(x, 0.0, available_rect),
        "2" => elevator.set_position(x, 200.0, available_rect),
        "1" => elevator.set_position(x, 400.0, available_rect),
        "G" => elevator.set_position(x, 600.0, available_rect),
        _ => (),
    }
}
fn at_floor_elevator_destination(elevator: &mut ElevatorObject, floor: &str, available_rect: egui::Rect) {
    match floor {
        "3" => elevator.set_destination(0.0, available_rect),
        "2" => elevator.set_destination(200.0, available_rect),
        "1" => elevator.set_destination(400.0, available_rect),
        "G" => elevator.set_destination(600.0, available_rect),
        _ => (),

    }
}
fn at_floor_person(person: &mut HumanObject, available_rect: egui::Rect) {
    match person.floor() {
        "3" => person.set_position(0.0, 0.0, available_rect),
        "2" => person.set_position(0.0, 200.0, available_rect),
        "1" => person.set_position(0.0, 400.0, available_rect),
        "G" => person.set_position(0.0, 600.0, available_rect),
        _ => (),
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
fn draw_buttons(ui: &mut egui::Ui, buttons: Vec<TextureHandle>, available_rect: egui::Rect, grid_size: f32) {
    let mut start_floors: f32 = available_rect.height() - (grid_size / 2.0);
    for button in &buttons {
        let position = egui::pos2(available_rect.min.x + 450.0, available_rect.min.x + start_floors);

        let rect = egui::Rect::from_min_size(position, egui::vec2(25.0, 50.0));
        let texture_id = button.id();
        ui.allocate_ui_at_rect(rect, |ui| {
            ui.painter().image(texture_id, rect, egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)), Color32::WHITE);
        });
        start_floors -= grid_size;

    }
}
fn draw_floors(ui: &mut egui::Ui, floors: Vec<String>, available_rect: egui::Rect, grid_size: f32) {
    let mut start_floor: f32 = available_rect.height() - (grid_size / 2.0);

    for floor in &floors {
        let position = egui::pos2(
            available_rect.min.x + 100.0,
            available_rect.min.y + start_floor - 50.0,
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
    println!("Elevator - Visualization and Analysis\n");
    eframe::run_native(
        "Elevator - Visualization and Analysis\n",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Box::new(Elevator::default())
        }),
    )
}
