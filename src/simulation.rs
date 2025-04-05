use egui::emath::easing::circular_in;
use rand::{random, rng, Rng};
use serde::{Deserialize, Serialize};
use std::f32::{self, consts::PI};
use std::fmt;
use strum_macros::EnumIter;

use crate::SmallestPolygonCoverApp;

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x: x, y: y }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Polygon {
    pub k: u8,              // Number of polygon nodes - körbeírandó poligon fokszáma
    pub nodes: Vec<Point>,  // Polygon nodes - poligon csúcsai
    pub circumference: f32, // Polygon circumference - poligon kerülete
}

impl Polygon {
    pub fn new(k: u8, nodes: Vec<Point>) -> Self {
        Self {
            k: k,
            nodes: nodes,
            circumference: 0_f32,
        }
    }
}

pub fn setup_points(n_o_points: u8) -> Vec<Point> {
    let mut points = vec![Point::default(); n_o_points as usize];
    let mut rnd = rand::rng();
    for i in 0..points.len() {
        // Ensures that the generated points are inside the circular area (x^2+y^2=r^2)
        // Biztosítja, hogy a pont ne kerüljön a körön kívüli területre az x^2+y^2=r^2 összefüggés segítségével
        let mut x = rnd.random_range(-0.5_f32..=0.5_f32);
        let mut y = rnd.random_range(-0.5_f32..=0.5_f32);
        if x.powi(2) > 0.25 - y.powi(2) {
            let radius = 0.25 - y.powi(2);
            x = rnd.random_range(-radius..radius);
        } else if y.powi(2) > 0.25 - x.powi(2) {
            let radius = 0.25 - x.powi(2);
            y = rnd.random_range(-radius..radius);
        }
        points[i].x = x;
        points[i].y = y;
    }
    points
}

pub fn setup_polygon(n_o_nodes: u8, random_polygon: bool) -> Polygon {
    let mut poly_nodes: Vec<Point> = vec![];
    let angle = (2_f32 * PI) / n_o_nodes as f32;
    let mut rnd_offset = 0_f32;
    if random_polygon {
        rnd_offset = rng().random_range(0_f32..(2_f32 * PI));
    }
    for i in 0..n_o_nodes as usize {
        poly_nodes.push(Point::new(
            (i as f32 * angle + rnd_offset).sin(),
            (i as f32 * angle + rnd_offset).cos(),
        ));
    }
    let polygon: Polygon = Polygon::new(n_o_nodes, poly_nodes);
    polygon
}

fn calculate_line_length(point1: &Point, point2: &Point) -> f32 {
    ((point1.x - point2.x).abs().powi(2) + (point1.y - point2.y).abs().powi(2)).sqrt()
}
fn caculate_circumference(polygon: &Polygon) -> f32 {
    let mut actual_circumference = 0_f32;
    for i in 0..polygon.nodes.len() {
        actual_circumference += calculate_line_length(
            &polygon.nodes[i],
            &polygon.nodes[(i + 1) % polygon.nodes.len()],
        );
    }
    actual_circumference
}

pub fn points_are_in_bounds(points: &[Point], polygon: &Polygon) -> bool {
    for i in 0..points.len() {
        if !point_is_in_bounds(points[i], polygon) {
            return false;
        } else {
            continue;
        }
    }
    return true;
}

fn point_is_in_bounds(point: Point, polygon: &Polygon) -> bool {
    for i in 0..polygon.nodes.len() {
        // If the lines are defined counter-clockwise:
        // - If it's 1, then it is "inside" of the line
        // - If it's 0, then it's on the line
        // - If it's -1, then it's "outside" of the line
        // Amennyiben vonalak iránya az óramutató járásával nem egyezik meg:
        // - Ha 1, akkor a pont a vonalon "belül" van
        // - Ha 0, akkor a pont a vonalon van
        // - Ha -1, akkor a vonalon "kívül" van
        let line = (
            polygon.nodes[i],
            polygon.nodes[(i + 1) % polygon.nodes.len()],
        );
        let position = ((point.x - line.0.x) * (line.1.y - line.0.y)
            - (point.y - line.0.y) * (line.1.x - line.0.x))
            .signum();
        if position < 0_f32 {
            // Returns false, because the point is inside the polygon
            // Hamis értéket ad vissza, amennyiben a pont a poligonon kívül van
            return false;
        } else {
            continue;
        }
    }
    // Returns true, because the point is inside the polygon
    // Igaz értéket ad vissza, amennyiben a pont a poligon belsejében van
    return true;
}

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize, EnumIter)]
pub enum Heuristics {
    Stochastic,
    SteepestAscentOneNode,
    SteepestAscentAllNodes,
    SCITerLimit,
    SCConstant,
    SCFitnessDependent,
}

impl fmt::Display for Heuristics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Heuristics::Stochastic => write!(f, "Stochastic"),
            Heuristics::SteepestAscentOneNode => write!(f, "Steepest ascent (one node)"),
            Heuristics::SteepestAscentAllNodes => write!(f, "Steepest ascent (all nodes)"),
            Heuristics::SCITerLimit => {
                write!(f, "Stochastic + Simulated cooling (iteration limit)")
            }
            Heuristics::SCConstant => write!(f, "Stochastic + Simulated cooling (constant)"),
            Heuristics::SCFitnessDependent => {
                write!(f, "Stochastic + Simulated cooling (fitness dependent)")
            }
        }
    }
}

pub fn execute_function(app: &mut SmallestPolygonCoverApp) -> Polygon {
    match app.selected_method {
        Heuristics::Stochastic => stochastic(
            &app.points,
            &mut app.polygon,
            app.stepsize,
            app.n_o_stop_generations,
            app.chance_of_bad_step,
            &mut app.stop_generation_counter,
            &mut app.circumference,
            &mut app.started,
        ),
        Heuristics::SteepestAscentOneNode => steepest_ascent(
            &app.points,
            &mut app.polygon,
            app.stepsize,
            app.search_resolution,
            app.n_o_stop_generations,
            &mut app.stop_generation_counter,
            &mut app.circumference,
            &mut app.started,
        ),
        Heuristics::SteepestAscentAllNodes => steepest_ascent_all(
            &app.points,
            &mut app.polygon,
            app.stepsize,
            app.search_resolution,
            app.n_o_stop_generations,
            &mut app.stop_generation_counter,
            &mut app.circumference,
            &mut app.started,
        ),
        Heuristics::SCITerLimit => sim_cooling_iter_limit(
            &app.points,
            &mut app.polygon,
            app.stepsize,
            app.temp_init,
            app.iter_limit,
            &mut app.temp,
            &mut app.circumference,
            &mut app.started,
        ),
        Heuristics::SCConstant => sim_cooling_const(
            &app.points,
            &mut app.polygon,
            app.stepsize,
            app.temp_init,
            app.temp_const,
            app.iter_const,
            app.n_o_stop_generations,
            &mut app.stop_generation_counter,
            &mut app.temp,
            &mut app.circumference,
            &mut app.started,
        ),
        Heuristics::SCFitnessDependent => sim_cooling_fit_dep(
            &app.points,
            &mut app.polygon,
            app.stepsize,
            app.temp_init,
            app.n_o_stop_generations,
            &mut app.stop_generation_counter,
            &mut app.temp,
            &mut app.circumference,
            &mut app.started,
        ),
    }
}

fn stochastic(
    points: &[Point],
    polygon: &mut Polygon,
    stepsize: f32,
    n_o_stop_generations: u8,
    chance_of_bad_step: f32,
    stop_generation_counter: &mut u8,
    circumference: &mut Vec<f32>,
    started: &mut bool,
) -> Polygon {
    let mut rnd = rand::rng();
    let mut actual_circumference = 0_f32;
    let n_o_nodes = polygon.nodes.len();
    if circumference.is_empty() {
        actual_circumference = caculate_circumference(&polygon);
        circumference.push(actual_circumference);
    }
    for i in 0..n_o_nodes {
        let original_point = polygon.nodes[i];
        let rand_degreee = rnd.random_range(0_f32..=(2_f32 * PI));
        let new_x_diff = rand_degreee.sin() * stepsize;
        let new_y_diff = rand_degreee.cos() * stepsize;
        polygon.nodes[i] = Point::new(original_point.x - new_x_diff, original_point.y - new_y_diff);
        actual_circumference = caculate_circumference(&polygon);
        if actual_circumference >= circumference[circumference.len() - 1]
            && rnd.random::<f32>() >= chance_of_bad_step
            || !points_are_in_bounds(points, &polygon)
        {
            polygon.nodes[i] = original_point;
        }
        actual_circumference = caculate_circumference(&polygon);
    }
    // Stopping condition - megállási feltétel
    if actual_circumference == *circumference.last().unwrap() {
        *stop_generation_counter += 1;
    } else if actual_circumference != *circumference.last().unwrap() {
        *stop_generation_counter = 0;
    }
    if *stop_generation_counter == n_o_stop_generations {
        *stop_generation_counter = 0;
        *started = false
    }
    circumference.push(actual_circumference);
    polygon.clone()
}

fn steepest_ascent(
    points: &[Point],
    polygon: &mut Polygon,
    stepsize: f32,
    search_resolution: u32,
    n_o_stop_generations: u8,
    stop_generation_counter: &mut u8,
    circumference: &mut Vec<f32>,
    started: &mut bool,
) -> Polygon {
    let mut rnd = rand::rng();
    let mut actual_circumference = 0_f32;
    let n_o_nodes = polygon.nodes.len();
    let random_index = rnd.random_range(0..polygon.k) as usize;
    let mut best_circumference = caculate_circumference(&polygon);
    let original_point = polygon.nodes[random_index];
    let mut best_point = original_point;
    for i in 0..search_resolution {
        let new_x_diff = (i as f32 * 2_f32 * PI / search_resolution as f32).sin() * stepsize;
        let new_y_diff = (i as f32 * 2_f32 * PI / search_resolution as f32).cos() * stepsize;
        polygon.nodes[random_index] =
            Point::new(original_point.x - new_x_diff, original_point.y - new_y_diff);
        actual_circumference = caculate_circumference(&polygon);
        if actual_circumference <= best_circumference && points_are_in_bounds(points, &polygon) {
            best_circumference = actual_circumference;
            best_point = polygon.nodes[random_index];
            polygon.nodes[random_index] = original_point;
        }
    }
    polygon.nodes[random_index] = best_point;
    actual_circumference = caculate_circumference(polygon);
    // Stopping condition - megállási feltétel
    if actual_circumference == *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter += 1;
    } else if actual_circumference != *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter = 0;
    }
    if *stop_generation_counter == n_o_stop_generations {
        *stop_generation_counter = 0;
        *started = false;
    }
    circumference.push(best_circumference);
    polygon.clone()
}

fn steepest_ascent_all(
    points: &[Point],
    polygon: &mut Polygon,
    stepsize: f32,
    search_resolution: u32,
    n_o_stop_generations: u8,
    stop_generation_counter: &mut u8,
    circumference: &mut Vec<f32>,
    started: &mut bool,
) -> Polygon {
    let mut rnd = rand::rng();
    let mut actual_circumference = 0_f32;
    let n_o_nodes = polygon.nodes.len();
    //let random_index = rnd.random_range(0..polygon.k) as usize;
    let mut best_circumference = caculate_circumference(&polygon);
    for i in 0..polygon.nodes.len() {
        let original_point = polygon.nodes[i];
        best_circumference = caculate_circumference(&polygon);
        let mut best_point = original_point;
        for j in 0..search_resolution {
            let new_x_diff = (j as f32 * 2_f32 * PI / search_resolution as f32).sin() * stepsize;
            let new_y_diff = (j as f32 * 2_f32 * PI / search_resolution as f32).cos() * stepsize;
            polygon.nodes[i] =
                Point::new(original_point.x - new_x_diff, original_point.y - new_y_diff);
            actual_circumference = caculate_circumference(&polygon);
            if actual_circumference <= best_circumference && points_are_in_bounds(points, &polygon)
            {
                best_circumference = actual_circumference;
                best_point = polygon.nodes[i];
                polygon.nodes[i] = original_point;
            }
        }
        polygon.nodes[i] = best_point;
    }
    actual_circumference = caculate_circumference(polygon);
    // Stopping condition - megállási feltétel
    if actual_circumference == *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter += 1;
    } else if actual_circumference != *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter = 0;
    }
    if *stop_generation_counter == n_o_stop_generations {
        *stop_generation_counter = 0;
        *started = false;
    }
    circumference.push(best_circumference);
    polygon.clone()
}

fn sim_cooling_iter_limit(
    points: &[Point],
    polygon: &mut Polygon,
    stepsize: f32,
    temp_init: f32,
    iter_limit: usize,
    temp: &mut Vec<f32>,
    circumference: &mut Vec<f32>,
    started: &mut bool,
) -> Polygon {
    let mut rnd = rand::rng();
    let mut actual_circumference = 0_f32;
    let n_o_nodes = polygon.nodes.len();
    if circumference.is_empty() {
        actual_circumference = caculate_circumference(&polygon);
        circumference.push(actual_circumference);
        temp.push(temp_init);
    }
    let mut new_temp = *temp.last().unwrap() - (temp_init / iter_limit as f32);
    if new_temp < 0_f32 {
        new_temp = 0_f32;
    }
    for i in 0..n_o_nodes {
        let original_point = polygon.nodes[i];
        let rand_degreee = rnd.random_range(0_f32..=(2_f32 * PI));
        let new_x_diff = rand_degreee.sin() * stepsize;
        let new_y_diff = rand_degreee.cos() * stepsize;
        polygon.nodes[i] = Point::new(original_point.x - new_x_diff, original_point.y - new_y_diff);
        actual_circumference = caculate_circumference(&polygon);
        if actual_circumference >= circumference[circumference.len() - 1]
            && rnd.random::<f32>() >= new_temp
            || !points_are_in_bounds(points, &polygon)
        {
            polygon.nodes[i] = original_point;
        }
        actual_circumference = caculate_circumference(&polygon);
    }
    // Stopping condition - megállási feltétel
    if circumference.len() >= iter_limit as usize {
        *started = false
    }
    circumference.push(actual_circumference);
    temp.push(new_temp);
    polygon.clone()
}

fn sim_cooling_const(
    points: &[Point],
    polygon: &mut Polygon,
    stepsize: f32,
    temp_init: f32,
    temp_const: f32,
    iter_const: usize,
    n_o_stop_generations: u8,
    stop_generation_counter: &mut u8,
    temp: &mut Vec<f32>,
    circumference: &mut Vec<f32>,
    started: &mut bool,
) -> Polygon {
    let mut rnd = rand::rng();
    let mut actual_circumference = 0_f32;
    let n_o_nodes = polygon.nodes.len();
    if circumference.is_empty() {
        actual_circumference = caculate_circumference(&polygon);
        circumference.push(actual_circumference);
        temp.push(temp_init);
    }
    let mut new_temp = *temp.last().unwrap() - (temp_const / iter_const as f32);
    if new_temp < 0_f32 {
        new_temp = 0_f32;
    }
    for i in 0..n_o_nodes {
        let original_point = polygon.nodes[i];
        let rand_degreee = rnd.random_range(0_f32..=(2_f32 * PI));
        let new_x_diff = rand_degreee.sin() * stepsize;
        let new_y_diff = rand_degreee.cos() * stepsize;
        polygon.nodes[i] = Point::new(original_point.x - new_x_diff, original_point.y - new_y_diff);
        actual_circumference = caculate_circumference(&polygon);
        if actual_circumference >= circumference[circumference.len() - 1]
            && rnd.random::<f32>() >= new_temp
            || !points_are_in_bounds(points, &polygon)
        {
            polygon.nodes[i] = original_point;
        }
        actual_circumference = caculate_circumference(&polygon);
    }
    // Stopping condition - megállási feltétel
    if actual_circumference == *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter += 1;
    } else if actual_circumference != *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter = 0;
    }
    if *stop_generation_counter == n_o_stop_generations {
        *stop_generation_counter = 0;
        *started = false;
    }
    circumference.push(actual_circumference);
    temp.push(new_temp);
    polygon.clone()
}

fn sim_cooling_fit_dep(
    points: &[Point],
    polygon: &mut Polygon,
    stepsize: f32,
    temp_init: f32,
    n_o_stop_generations: u8,
    stop_generation_counter: &mut u8,
    temp: &mut Vec<f32>,
    circumference: &mut Vec<f32>,
    started: &mut bool,
) -> Polygon {
    let mut rnd = rand::rng();
    let mut actual_circumference = 0_f32;
    let n_o_nodes = polygon.nodes.len();
    if circumference.is_empty() {
        actual_circumference = caculate_circumference(&polygon);
        circumference.push(actual_circumference);
        temp.push(temp_init);
    }
    let mut new_temp = *temp.last().unwrap();
    if circumference.len() > 2 {
        new_temp -= *temp.last().unwrap()
            * (1_f32
                - (circumference
                    .get(circumference.len() - 1)
                    .unwrap_or(&1_f32)
                    .min(*circumference.get(circumference.len() - 2).unwrap_or(&1_f32))
                    / circumference
                        .get(circumference.len() - 2)
                        .unwrap_or(&1_f32)
                        .max(*circumference.get(circumference.len() - 1).unwrap_or(&1_f32))));
    }
    if new_temp < 0_f32 {
        new_temp = 0_f32;
    }
    for i in 0..n_o_nodes {
        let original_point = polygon.nodes[i];
        let rand_degreee = rnd.random_range(0_f32..=(2_f32 * PI));
        let new_x_diff = rand_degreee.sin() * stepsize;
        let new_y_diff = rand_degreee.cos() * stepsize;
        polygon.nodes[i] = Point::new(original_point.x - new_x_diff, original_point.y - new_y_diff);
        actual_circumference = caculate_circumference(&polygon);
        if actual_circumference > circumference[circumference.len() - 1]
            && rnd.random::<f32>() >= new_temp
            || !points_are_in_bounds(points, &polygon)
        {
            polygon.nodes[i] = original_point;
        }
        actual_circumference = caculate_circumference(&polygon);
    }
    // Stopping condition - megállási feltétel
    if actual_circumference == *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter += 1;
    } else if actual_circumference != *circumference.last().unwrap_or(&0_f32) {
        *stop_generation_counter = 0;
    }
    if *stop_generation_counter == n_o_stop_generations {
        *stop_generation_counter = 0;
        *started = false;
    }
    circumference.push(actual_circumference);
    temp.push(new_temp);
    polygon.clone()
}
