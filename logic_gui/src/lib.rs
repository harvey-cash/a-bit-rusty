use eframe::{egui, App, Frame};
use egui::Vec2; // Only Vec2 is directly used from egui here now
use std::collections::HashMap;

use logic_core::{chip::types::{NodeType, NodeTypeMap, PinLayout}, node_type_map}; // Updated imports

pub mod chip_view;
use chip_view::ChipView;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct MyApp {
    chip_display: ChipView,
    sample_pin_layout: PinLayout,
    sample_logic_levels: HashMap<usize, u8>,
}

impl Default for MyApp {
    fn default() -> Self {
        let id_type_map: NodeTypeMap = node_type_map!(
            0 => NodeType::Ground,
            1 => NodeType::Supply,
            2 => NodeType::Input,
            3 => NodeType::Input,
            4 => NodeType::Output,
        );

        let mut logic_levels = HashMap::new();
        logic_levels.insert(0, 0);
        logic_levels.insert(1, 1);
        logic_levels.insert(2, 1);
        logic_levels.insert(3, 0);
        logic_levels.insert(4, 0);

        Self {
            chip_display: ChipView::new(),
            sample_pin_layout: PinLayout::new(id_type_map),
            sample_logic_levels: logic_levels,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Digital Chip Simulator");
            ui.separator();

            let chip_position = ui.available_rect_before_wrap().min + Vec2::new(50.0, 50.0);
            
            self.chip_display.show(
                ui,
                "74LS00 NAND",
                &self.sample_pin_layout,
                &self.sample_logic_levels,
                chip_position,
                "my_sample_chip"
            );
        });
    }
}