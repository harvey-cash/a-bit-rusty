use eframe::{egui, App, Frame};
use egui::Vec2;

use logic_core::{chip::{chip::ChipType, designer::{ChipPinLink, Designer, DesignerState}, types::{PinLayout, ChipAndPin}}, chip_pin}; // Updated imports

pub mod chip_view;
use chip_view::ChipView;

#[derive(serde::Serialize)]
#[serde(default)]
pub struct MyApp {
    designer_state: DesignerState
}

impl Default for MyApp {
    fn default() -> Self {

        let mut designer = Designer::new();
        let input = designer.add_chip(&ChipType::Input.to_string()).unwrap();
        let output = designer.add_chip(&ChipType::Output.to_string()).unwrap();
        let _ = designer.add_link(ChipPinLink { 
            source: chip_pin!(input, 0), 
            target: chip_pin!(output, 0) 
        });
        let _ = designer.set_input_chip_value(input, 1);
        designer.tick();
        let state = designer.get_state();

        Self {
            designer_state: state
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Digital Chip Simulator");
            ui.separator();

            let mut pos_x = 0.0;
            for (id, name) in &self.designer_state.chip_names {
                pos_x += 50.0;
                let chip_position = ui.available_rect_before_wrap().min + Vec2::new(pos_x, 50.0);
            
                let chip_display = ChipView::new();

                let layout: &PinLayout = self.designer_state.layouts.get(name).unwrap();
                let logic_levels = self.designer_state.chip_pin_states.get(id).unwrap();

                chip_display.show(
                    ui,
                    "74LS00 NAND",
                    layout,
                    logic_levels,
                    chip_position,
                    "my_sample_chip"
                );
            }

            
        });
    }
}