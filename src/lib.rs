use std::sync::Arc;

use anyhow::Result;
use eframe::egui::{self, ViewportBuilder};
use tracing::{debug, info, instrument};

// Constants
const LIBRARY_NAME: &str = "cgol-rs";
const TITLE: &str = "Conway's Game of Life - rs";
const APP_NAME: &str = "cgol-rs";
const INITIAL_GRID_SIZE: (u32, u32) = (100, 100);

#[instrument]
pub fn main() -> Result<()> {
    info!("{LIBRARY_NAME} library invoked, starting");

    info!("Configuring eframe");
    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: ViewportBuilder {
            app_id: Some(APP_NAME.to_string()),
            title: Some(TITLE.to_string()),
            icon: Some(Arc::new(egui::IconData::default())),
            ..Default::default()
        },
        ..Default::default()
    };

    info!("Starting eframe");
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )?;

    info!("Finished cgol-rs library successfully");
    Ok(())
}

#[derive(Default)]
struct App {}

impl App {
    
    #[instrument(skip_all)]
    fn new(_: &eframe::CreationContext<'_>) -> Self {
        info!("Configuring App");
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for App {
    
    #[instrument(skip_all)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        debug!("Rendering App UI");
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Shapes Example");

            // Allocate space for our custom drawing area (e.g., 200x200 pixels)
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(200.0, 200.0), egui::Sense::click());

            let painter = ui.painter();

            // 1. Draw a filled circle
            let center = rect.center();
            painter.circle_filled(center, 50.0, egui::Color32::from_rgb(100, 200, 100));

            // 2. Draw a hollow rectangle outline
            let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 255, 255));
            painter.rect_stroke(rect, 5.0, stroke, egui::StrokeKind::Outside);

            // 3. Draw a line from top-left to bottom-right
            let line_points = [rect.min, rect.max];
            painter.line_segment(line_points, egui::Stroke::new(3.0, egui::Color32::RED));
        });
    }
}
