use std::sync::Arc;

use anyhow::Result;
use eframe::egui::{self, ViewportBuilder};
use tracing::{info, instrument};

// Constants
const TITLE: &str = "Conway's Game of Life - rs";
const APP_NAME: &str = "cgol-rs";
const CELL_SIZE: f32 = 6.0;
const ROWS: usize = 100;
const COLUMNS: usize = 100;
const SEED: [(usize, usize); 5] = [(60, 61), (60, 62), (61, 60), (61, 61), (62, 61)]; //Trying R-pentomino

#[instrument]
pub fn main() -> Result<()> {
    info!("cgol-rs library invoked, starting");

    info!("Configuring eframe");
    let padding: f32 = 100.0;
    let min_width = (COLUMNS as f32 * CELL_SIZE) + padding;
    let min_height = (ROWS as f32 * CELL_SIZE) + padding;
    info!("of size ({min_width}, {min_height})");

    let native_options = eframe::NativeOptions {
        centered: true,
        viewport: ViewportBuilder {
            app_id: Some(APP_NAME.to_string()),
            title: Some(TITLE.to_string()),
            icon: Some(Arc::new(egui::IconData::default())),
            inner_size: Some(eframe::egui::vec2(min_width, min_height)),
            min_inner_size: Some(eframe::egui::vec2(min_width, min_height)),
            resizable: Some(true),
            ..Default::default()
        },
        ..Default::default()
    };

    info!("Starting eframe");
    eframe::run_native(
        APP_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, min_width / 4.0)))),
    )?;

    info!("Finished cgol-rs library successfully");
    Ok(())
}

struct Grid {
    rows: usize,
    columns: usize,
    cells: Vec<Vec<bool>>,

    /// The generation of the cells
    generation: u32,
}

impl Grid {
    fn new() -> Self {
        let mut grid = Self {
            rows: ROWS,
            columns: COLUMNS,
            cells: vec![vec![false; COLUMNS as usize]; ROWS as usize],
            generation: 0,
        };

        //Initializing with seed
        for (x, y) in SEED {
            grid.cells[x][y] = true;
        }
        return grid;
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new()
    }
}

#[derive(Default)]
struct App {
    prev_grid: Grid,
    cur_grid: Grid,
    left_margin: f32,
}

impl App {
    #[instrument(skip_all)]
    fn new(_: &eframe::CreationContext<'_>, left_margin: f32) -> Self {
        info!("Configuring App");

        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        Self {
            left_margin,
            ..Default::default()
        }
    }
}

impl eframe::App for App {
    #[instrument(skip_all)]
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        let should_proceed_animation: bool = !ui.requested_repaint_last_pass();

        //Updating grid
        if should_proceed_animation {
            //Updating prev grid to cur grid, swapping here as it is more efficient to simply swap the memories as we are going to be updating the cur grid and updating each cell in it anyway
            std::mem::swap(&mut self.prev_grid.cells, &mut self.cur_grid.cells);
            for (i, _) in (0..self.prev_grid.rows).enumerate() {
                for (j, _) in (0..self.prev_grid.columns).enumerate() {
                    let new_cell_state = should_fill(i, j, &self.prev_grid);
                    self.cur_grid.cells[i][j] = new_cell_state;
                }
            }

            //Bump the generation counter
            self.cur_grid.generation += 1;
        }

        //UI Logic
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Conway's Game of Life");
                ui.add_space(80.0);
                ui.horizontal(|ui| {
                    ui.add_space(self.left_margin);
                    ui.vertical_centered(|ui| {
                        for (i, _) in (0..ROWS).enumerate() {
                            ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                            ui.horizontal_top(|ui| {
                                ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);

                                for (j, _) in (0..COLUMNS).enumerate() {
                                    let (rect, _) = ui.allocate_exact_size(
                                        egui::vec2(CELL_SIZE, CELL_SIZE),
                                        egui::Sense::hover(),
                                    );

                                    let fill_status = self.cur_grid.cells[i][j];

                                    let painter = ui.painter();
                                    painter.rect_filled(
                                        rect,
                                        0.0,
                                        match fill_status {
                                            true => egui::Color32::WHITE,
                                            false => egui::Color32::BLACK,
                                        },
                                    );
                                }
                            });
                        }
                    });
                });

                ui.label(format!("Generation: {}", self.cur_grid.generation));
            });
        });

        //Proceed the animation, this function being called raises a flag that the UI was not requested to be repainted by the
        //user, hence and with the 1s duration, the animation proceeds at a rate of 1 frame per second.
        ui.request_repaint_after(std::time::Duration::from_millis(300));
    }
}

/// The main function that determines whether a cell should be painted or not.
///
/// CGOL has 4 primary rules, however, it applies to a grid that is infinite.
/// As such, we have to determine what to do with the cells at the edges.
///
/// There are some techniques that are used for this, and we are going to use one called
/// Toroidal wrapping. In this technique, the grid is treated as if it were wrapped around a torus,
/// meaning that the cells on the edges are considered to be adjacent to the cells on the opposite edges.
/// This allows for a continuous and infinite-like behavior in a finite grid.
fn should_fill(x: usize, y: usize, grid: &Grid) -> bool {
    let n_count = get_neighbor_count(x, y, grid);
    let is_alive = grid.cells[x][y];
    //CGOL Rules

    // Every cell counts how many of its 8 neighbors (horizontal, vertical, and diagonal) are currently alive.
    // The 4 Rules
    //1. Underpopulation: Any live cell with fewer than 2 live neighbors dies.
    //2. Survival: Any live cell with exactly 2 or 3 live neighbors lives on to the next generation.
    //3. Overpopulation: Any live cell with more than 3 live neighbors dies.
    //4. Reproduction: Any dead cell with exactly exactly 3 live neighbors becomes a live cell
    if is_alive {
        //Cell is alive right now
        match n_count {
            ..2 => false,
            2 | 3 => true,
            3.. => false,
        }
    } else {
        //Cell is dead right now
        match n_count {
            3 => true,
            _ => false,
        }
    }
}

fn get_neighbor_count(x: usize, y: usize, grid: &Grid) -> usize {
    let x = x as i32;
    let y = y as i32;
    let rows = grid.rows as i32;
    let columns = grid.columns as i32;
    let xmin = ((x - 1 + rows) % rows) as usize;
    let xcur = x as usize;
    let xmax = ((x + 1) % rows) as usize;

    let ymin = ((y - 1 + columns) % columns) as usize;
    let ycur = y as usize;
    let ymax = ((y + 1) % columns) as usize;

    [
        grid.cells[xmin][ymin],
        grid.cells[xmin][ycur],
        grid.cells[xmin][ymax],
        grid.cells[xcur][ymin],
        //Don't count the current cell's state (xcur,ycur)
        grid.cells[xcur][ymax],
        grid.cells[xmax][ymin],
        grid.cells[xmax][ycur],
        grid.cells[xmax][ymax],
    ]
    .iter()
    .filter(|elem| **elem)
    .count()
}

//First we need a grid
//Next we need to be able to reach any x,y cell in that grid
//Next we need to define the state of each cell and be able to manipulate it, toggling it either off or on based on it being filled
//For the cell rules we need to have a global state of the grid
//Next for each cell x,y we then decide whether it is painted or not based on cgol rules
//and so we do for each cell in the grid

//The global painter paints the whole grid and checks whether each cell is either filled or unfilled
