use logic_gui::MyApp;

fn main() -> Result<(), eframe::Error> {
    // Initialize the logger for native execution.
    // You can customize the log level, e.g., by setting the RUST_LOG environment variable.
    // Example: RUST_LOG=info cargo run -p logic_gui
    env_logger::init(); 
    log::info!("Egui MVP native app starting...");

    let native_options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]) // Default window size
            .with_min_inner_size([300.0, 220.0]), // Minimum window size
        ..Default::default()
    };

    eframe::run_native(
        "Egui MVP Rectangle (Native)", // Window title
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))), // Create our app state
    )
}