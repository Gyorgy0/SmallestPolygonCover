use egui::{self, CentralPanel, Color32, Pos2, Stroke, Ui, Visuals, Widget};
use egui_plot::{Legend, Line, PlotPoints};
use std::{f32::consts::PI, ops::RangeInclusive, u32, vec};
use strum::IntoEnumIterator;

use crate::simulation::{
    execute_function, setup_points, setup_polygon, Heuristics, Point, Polygon,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct SmallestPolygonCoverApp {
    #[serde(skip)]
    pub points: Vec<Point>,
    #[serde(skip)]
    pub polygon: Polygon,
    pub selected_method: Heuristics,
    pub n_o_points: u8,
    pub n_o_nodes: u8,
    pub n_o_stop_generations: u8,
    #[serde(skip)]
    pub stop_generation_counter: u8,
    pub search_resolution: u32,
    #[serde(skip)]
    pub stuck: bool,
    pub n_o_searches: u32,
    #[serde(skip)]
    pub search_counter: u32,
    #[serde(skip)]
    pub temp: Vec<f32>,
    #[serde(skip)]
    pub circumference: Vec<f32>,
    pub stepsize: f32,
    #[serde(skip)]
    pub started: bool,
}

impl Default for SmallestPolygonCoverApp {
    fn default() -> Self {
        Self {
            points: vec![],
            polygon: Polygon::default(),
            selected_method: Heuristics::SteepestAscent,
            n_o_points: 0,
            n_o_nodes: 3,
            n_o_stop_generations: 0,
            stop_generation_counter: 0,
            search_resolution: 100,
            stuck: false,
            n_o_searches: 1,
            search_counter: 0,
            temp: vec![],
            circumference: vec![],
            stepsize: 0.01_f32,
            started: false,
        }
    }
}

impl SmallestPolygonCoverApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        cc.egui_ctx.set_visuals(Visuals::light());
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl eframe::App for SmallestPolygonCoverApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(&ctx, |ui| {
            egui::Window::new("Options (beállítások)").show(ctx, |ui| {
                let mut options_ui = egui::UiBuilder::new();
                if self.started {
                    options_ui = options_ui.disabled();
                }
                ui.scope_builder(options_ui, |ui| {
                    ui.label("Number of points (pontok száma):");
                    egui::Slider::new(&mut self.n_o_points, RangeInclusive::new(0, 255)).ui(ui);
                    ui.label("Number of nodes (csúcsok száma):");
                    egui::Slider::new(&mut self.n_o_nodes, RangeInclusive::new(3, 255)).ui(ui);
                    ui.label("Step size (lépés nagysága):");
                    egui::Slider::new(&mut self.stepsize, RangeInclusive::new(0_f32, 1_f32)).ui(ui);
                    ui.label("Search method (keresési módszer):");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{}", self.selected_method.to_string()))
                        .show_ui(ui, |ui| {
                            Heuristics::iter().for_each(|method| {
                                ui.selectable_value(
                                    &mut self.selected_method,
                                    method,
                                    method.to_string(),
                                );
                            });
                        });
                    ui.separator();
                    ui.label("Stopping conditions (megállási feltételek)");
                    // Stochastic method stopping conditions - sztohasztikus módszer megállási feltétele
                    if self.selected_method == Heuristics::Stochastic {
                        ui.label(
                            "Number of not improving generations (nem javuló generációk száma):",
                        );
                        // n_o_stop_generations - this variable specifies how much generations are allowed that are not better than the previous
                        // n_o_stop_generation - ez a változó megadja mennyi generáció lehet, amely nem jobb, mint az előző
                        egui::Slider::new(
                            &mut self.n_o_stop_generations,
                            RangeInclusive::new(0_u8, u8::MAX),
                        )
                        .ui(ui);
                    }
                    if self.selected_method == Heuristics::SteepestAscent {
                        ui.label("Search resolution (keresés részletessége):");
                        // search_resolution - this variable defines how many point do we need to look for
                        egui::Slider::new(
                            &mut self.search_resolution,
                            RangeInclusive::new(1_u32, u32::MAX),
                        )
                        .ui(ui);
                    }
                    if self.selected_method == Heuristics::RandomRestart {
                        ui.label("Number of searches (keresések száma):");
                        // n_o_searches - this variable specifies how much searches do we need to start
                        // n_o_searches - ez a változó megadja az indítandó keresések számát
                        egui::Slider::new(
                            &mut self.n_o_searches,
                            RangeInclusive::new(0_u32, u32::MAX),
                        )
                        .ui(ui);
                    }
                    ui.separator();
                    if ui.button("Generate").clicked() {
                        self.circumference = vec![];
                        self.started = false;
                        self.points = setup_points(self.n_o_points);
                        self.polygon = setup_polygon(self.n_o_nodes);
                    }
                });
                if !self.started {
                    if ui.button("Start").clicked() {
                        self.started = true;
                    }
                } else if self.started {
                    self.polygon = execute_function(self);
                    if ui.button("Stop").clicked() {
                        self.started = false;
                    }
                }

                if ui.button("Next generation").clicked() {
                    self.polygon = execute_function(self);
                }
                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
            });
            egui::Window::new("Circumference (kerület)").show(ctx, |ui| {
                ui.label("Author: Juraj Lukovics");
                ui.label(egui::special_emojis::GITHUB.to_string() + " GitHub: ");
                ui.add(egui::Hyperlink::new("https://github.com/Gyorgy0"));
                egui_plot::Plot::new("Plot")
                    .allow_drag(true)
                    .legend(Legend::default())
                    .show(ui, |plot_ui| {
                        let points = PlotPoints::from_ys_f32(&self.circumference);
                        plot_ui.line(Line::new(points));
                    });
            });
            display_contents(self, ctx, ui);
            ctx.request_repaint();
        });
    }
}

fn display_contents(app: &mut SmallestPolygonCoverApp, ctx: &egui::Context, ui: &mut Ui) {
    // Center of the window - Az ablak közepe
    let center: Pos2 = Pos2::new(
        ctx.screen_rect().width() / 2.0,
        ctx.screen_rect().height() / 2.0,
    );
    // Radius of the circle covered by the window - Az ablak belülírt körének sugara
    let radius = center.x.min(center.y);
    ui.painter().circle(
        Pos2::new(
            ctx.screen_rect().width() / 2.0,
            ctx.screen_rect().height() / 2.0,
        ),
        radius,
        Color32::from_rgba_unmultiplied(0, 0, 0, 0),
        Stroke::new(15.0, Color32::from_rgba_unmultiplied(255, 0, 0, 50)),
    );
    // Displaying the polygon - Poligon megjelenítése
    let mut points: Vec<Pos2> = vec![];
    for i in 0..app.polygon.nodes.len() {
        points.push(Pos2::new(
            center.x + (radius * app.polygon.nodes[i].x),
            center.y + (radius * app.polygon.nodes[i].y),
        ));
        points.push(Pos2::new(
            center.x + (radius * app.polygon.nodes[(i + 1) % app.polygon.nodes.len()].x),
            center.y + (radius * app.polygon.nodes[(i + 1) % app.polygon.nodes.len()].y),
        ));
    }
    ui.painter().line(points, Stroke::new(5.0, Color32::RED));
    // Displaying points - Pontok megjelenítése
    for i in 0..app.points.len() {
        ui.painter().circle(
            Pos2::new(
                (ctx.screen_rect().width() / 2.0) + (radius * app.points[i].x),
                (ctx.screen_rect().height() / 2.0) + (radius * app.points[i].y),
            ),
            2.5,
            Color32::from_rgba_unmultiplied(0, 0, 0, 50),
            Stroke::new(1.0, Color32::BLACK),
        );
    }
}
