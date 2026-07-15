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
            min_inner_size: Some(eframe::egui::vec2(400.0, 400.0)),
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
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        debug!("Rendering App UI");
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Conway's Game of Life");

                let cell_size = 12.0;
                let columns = 40;
                let frame_width = columns as f32 * cell_size;
                let left_margin = (ui.available_width() - frame_width) / 2.0;

                ui.horizontal(|ui| {
                    ui.add_space(left_margin.max(0.0));

                    egui::Frame::default()
                        .stroke(egui::Stroke::new(1.0, egui::Color32::BLACK))
                        .show(ui, |ui| {
                            ui.horizontal_top(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                                for (i, _) in (0..columns).enumerate() {
                                    let (rect, _) = ui.allocate_exact_size(
                                        egui::vec2(cell_size, cell_size),
                                        egui::Sense::hover(),
                                    );
                                    let painter = ui.painter();
                                    painter.rect_filled(
                                        rect,
                                        0.0,
                                        if i % 2 == 0 {
                                            egui::Color32::RED
                                        } else {
                                            egui::Color32::GREEN
                                        },
                                    );
                                }
                            });
                        });
                });
            });

            /*      // 1. Draw a filled circle
            let center = rect.center();
            painter.circle_filled(center, 50.0, egui::Color32::from_rgb(100, 200, 100)); */
            /*
            // 2. Draw a hollow rectangle outline
            let stroke = egui::Stroke::new(2.0, egui::Color32::from_rgb(255, 255, 255));
            painter.rect_stroke(rect, 5.0, stroke, egui::StrokeKind::Outside);

            // 3. Draw a line from top-left to bottom-right
            let line_points = [rect.min, rect.max];
            painter.line_segment(line_points, egui::Stroke::new(3.0, egui::Color32::RED)); */
        });
    }
}
struct Grid {
    cells: Vec<Vec<eframe::egui::Rect>>,
}
impl Grid {
    /* fn new() -> Self {
        Self {
            cells: Vec::new(Vec::new(eframe::egui::Rect::))
        }
    } */
}
/* fn should_fill(grid) */

//First we need a grid
//Next we need to be able to reach any x,y cell in that grid
//Next we need to define the state of each cell and be able to manipulate it, toggling it either off or on based on it being filled
//For the cell rules we need to have a global state of the grid
//Next for each cell x,y we then decide whether it is painted or not based on cgol rules
//and so we do for each cell in the grid

//The global painter paints the whole grid and checks whether each cell is either filled or unfilled
