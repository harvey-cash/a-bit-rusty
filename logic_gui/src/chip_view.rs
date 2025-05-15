use eframe::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2, Align2, FontId};
use egui::CornerRadius;
use std::collections::HashMap;
use logic_core::chip::types::PinLayout; // Import PinLayout from the logic_core crate

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ChipView {
    // View-specific state or styling can go here in the future
}

impl ChipView {
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn show(
        &self,
        ui: &mut egui::Ui,
        chip_name: &str,
        layout: &PinLayout,
        logic_levels: &HashMap<usize, u8>,
        position: Pos2,
        chip_id_source: impl std::hash::Hash,
    ) -> egui::Response {
        // --- Visual Parameters (same as before) ---
        const CHIP_BODY_WIDTH: f32 = 150.0;
        const CHIP_BODY_HEIGHT: f32 = 200.0;
        const PIN_SIZE: f32 = 10.0;
        const PIN_SPACING: f32 = 5.0;
        const EDGE_MARGIN: f32 = 15.0;

        let color_pin_low = Color32::from_gray(50);
        let color_pin_high = Color32::YELLOW;
        let color_pin_ground = Color32::DARK_BLUE;
        let color_pin_supply = Color32::DARK_RED;
        let color_chip_body = Color32::from_gray(75);
        let color_chip_border = Color32::from_gray(120);
        let color_text = Color32::WHITE;

        // --- Chip Body (same as before) ---
        let chip_rect = Rect::from_min_size(position, Vec2::new(CHIP_BODY_WIDTH, CHIP_BODY_HEIGHT));
        let response = ui.interact(chip_rect, ui.id().with(chip_id_source), Sense::click_and_drag());

        ui.painter().rect_filled(chip_rect, CornerRadius::same(5), color_chip_body);
        ui.painter().rect_stroke(chip_rect, CornerRadius::same(5), Stroke::new(1.5, color_chip_border), egui::StrokeKind::Middle);

        // --- Chip Name (same as before) ---
        ui.painter().text(
            chip_rect.center_top() + Vec2::new(0.0, EDGE_MARGIN + 5.0),
            Align2::CENTER_CENTER,
            chip_name,
            FontId::proportional(16.0),
            color_text,
        );

        // --- Helper to draw a row/column of pins (same as before) ---
        let draw_pins = |
            ui_painter: &egui::Painter,
            pin_ids: &[usize],
            is_horizontal: bool,
            start_pos: Pos2,
            default_color: Color32,
        | {
            // Renamed current_pos to avoid shadowing for clarity, though original was fine
            for (i, pin_id) in pin_ids.iter().enumerate() {
                let pin_center = if is_horizontal {
                    start_pos + Vec2::new(i as f32 * (PIN_SIZE + PIN_SPACING) + PIN_SIZE / 2.0, PIN_SIZE / 2.0)
                } else {
                    start_pos + Vec2::new(PIN_SIZE / 2.0, i as f32 * (PIN_SIZE + PIN_SPACING) + PIN_SIZE / 2.0)
                };

                let pin_rect = Rect::from_center_size(pin_center, Vec2::splat(PIN_SIZE));
                let logic_level = logic_levels.get(pin_id).copied().unwrap_or(0);
                
                let pin_color = match logic_level {
                    1 => color_pin_high,
                    0 => color_pin_low,
                    _ => default_color,
                };
                let final_pin_color = if layout.ground_pins.contains(pin_id) {
                    color_pin_ground
                } else if layout.supply_pins.contains(pin_id) {
                    color_pin_supply
                } else {
                    pin_color
                };

                ui_painter.rect_filled(pin_rect, CornerRadius::ZERO, final_pin_color);
                ui_painter.rect_stroke(pin_rect, CornerRadius::ZERO, Stroke::new(1.0, Color32::BLACK), egui::StrokeKind::Middle);
            }
        };

        // --- Draw Pins (same logic as before) ---
        let top_pins_combined = [&layout.ground_pins[..], &layout.supply_pins[..]].concat();
        if !top_pins_combined.is_empty() {
            let num_top_pins = top_pins_combined.len();
            let total_width_top_pins = num_top_pins as f32 * PIN_SIZE + (num_top_pins.saturating_sub(1)) as f32 * PIN_SPACING;
            let top_pins_start_x = chip_rect.left() + (CHIP_BODY_WIDTH - total_width_top_pins) / 2.0;
            draw_pins(
                ui.painter(),
                &top_pins_combined,
                true,
                Pos2::new(top_pins_start_x, chip_rect.top() - PIN_SIZE),
                color_pin_low,
            );
        }

        if !layout.input_pins.is_empty() {
            let num_left_pins = layout.input_pins.len();
            let total_height_left_pins = num_left_pins as f32 * PIN_SIZE + (num_left_pins.saturating_sub(1)) as f32 * PIN_SPACING;
            let left_pins_start_y = chip_rect.top() + (CHIP_BODY_HEIGHT - total_height_left_pins) / 2.0;
            draw_pins(
                ui.painter(),
                &layout.input_pins,
                false,
                Pos2::new(chip_rect.left() - PIN_SIZE, left_pins_start_y),
                color_pin_low,
            );
        }
        
        if !layout.output_pins.is_empty() {
            let num_right_pins = layout.output_pins.len();
            let total_height_right_pins = num_right_pins as f32 * PIN_SIZE + (num_right_pins.saturating_sub(1)) as f32 * PIN_SPACING;
            let right_pins_start_y = chip_rect.top() + (CHIP_BODY_HEIGHT - total_height_right_pins) / 2.0;
            draw_pins(
                ui.painter(),
                &layout.output_pins,
                false,
                Pos2::new(chip_rect.right(), right_pins_start_y),
                color_pin_low,
            );
        }

        response
    }
}