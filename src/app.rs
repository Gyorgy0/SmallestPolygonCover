use std::ops::{Not, RangeInclusive};

use egui::{self, CentralPanel, Color32, Pos2, Stroke, Ui, Vec2, Visuals, Widget};
use egui_plot::{Legend, Line, PlotPoints};

use crate::simulation::{setupPoints, setupPolygon, Point, Polygon};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    generation: u64,
    points: Vec<Point>,
    polygon: Polygon,
    n_o_points: u8,
    n_o_nodes: u8,
    circumference: f32,
    stepsize: f32,
    fitness: f32,
    started: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            generation: 0,
            points: vec![Point::default(); 0],
            polygon: Polygon::default(),
            n_o_points: 0,
            n_o_nodes: 3,
            circumference: 0_f32,
            stepsize: 0.01,
            fitness: 0_f32,
            started: false,
        }
    }
}

impl TemplateApp {
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

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(&ctx, |ui| {
            // Painting with a painter on centralpanel
            //ui.painter().line(vec![Pos2::new(0.0, 0.0), Pos2::new(50.0, 50.0)], PathStroke::new(10.0, Color32::RED));
            egui::Window::new("Options").show(ctx, |ui| {
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
                    if ui.button("Generate").clicked() {
                        self.points = setupPoints(self.n_o_points);
                        self.polygon = setupPolygon(self.n_o_nodes);
                    }
                });
                if !self.started {
                    if ui.button("Start").clicked() {
                        self.started = true;
                    }
                } else if self.started {
                    if ui.button("Stop").clicked() {
                        self.started = false;
                    }
                }
                if ui.button("Next generation").clicked() {}
                if ui.button("Reset").clicked() {
                    *self = Self::default();
                }
            });
            display_contents(self, ctx, ui);
            ctx.request_repaint();
        });
    }
}

fn display_contents(app: &mut TemplateApp, ctx: &egui::Context, ui: &mut Ui) {
    // Radius of the circle covered, by the window
    let mut radius = 0_f32;
    //Center of the window
    let center: Pos2 = Pos2::new(
        ctx.screen_rect().width() / 2.0,
        ctx.screen_rect().height() / 2.0,
    );
    if ctx.screen_rect().height() < ctx.screen_rect().width() {
        radius = ctx.screen_rect().height() / 2.0;
    } else {
        radius = ctx.screen_rect().width() / 2.0;
    }
    ui.painter().circle(
        Pos2::new(
            ctx.screen_rect().width() / 2.0,
            ctx.screen_rect().height() / 2.0,
        ),
        radius,
        Color32::from_rgba_unmultiplied(0, 0, 0, 0),
        Stroke::new(15.0, Color32::from_rgba_unmultiplied(255, 0, 0, 50)),
    );
    // Displaying points
    for i in 0..app.points.len() {
        ui.painter().circle(
            Pos2::new(
                (ctx.screen_rect().width() / 2.0) + ((radius / 2.0) * app.points[i].x),
                (ctx.screen_rect().height() / 2.0) + ((radius / 2.0) * app.points[i].y),
            ),
            1.0,
            Color32::from_rgba_unmultiplied(0, 0, 0, 10),
            Stroke::new(1.0, Color32::BLACK),
        );
    }
    // Displaying the polygon
    let mut points: Vec<Pos2> = vec![];
    for i in 0..app.polygon.lines.len() {
        points.push(Pos2::new(
            center.x + radius * app.polygon.lines[i].point1.x,
            center.y + radius * app.polygon.lines[i].point1.y,
        ));
        points.push(Pos2::new(
            center.x + radius * app.polygon.lines[i].point2.x,
            center.y + radius * app.polygon.lines[i].point2.y,
        ));
    }
    ui.painter().line(points, Stroke::new(5.0, Color32::RED));

    /*egui::Window::new("Graph").show(ctx, |ui| {
        egui_plot::Plot::new("Plot")
            .allow_zoom(true)
            .allow_drag(true)
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                let points = PlotPoints::from_explicit_callback(|x| x.powi(2), .., 10000);
                plot_ui.line(Line::new(points));
            });
    });*/
}
