use rand::Rng;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;

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
    pub k: u8,             // Number of polygon nodes - körbeírandó poligon fokszáma
    pub nodes: Vec<Point>, // Polygon nodes - poligon csúcsai
}

impl Polygon {
    pub fn new(k: u8, nodes: Vec<Point>) -> Self {
        Self { k: k, nodes: nodes }
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

pub fn setup_polygon(n_o_nodes: u8) -> Polygon {
    let mut poly_nodes: Vec<Point> = vec![];
    let angle = (2_f32 * PI) / n_o_nodes as f32;
    for i in 0..n_o_nodes as usize {
        poly_nodes.push(Point::new(
            (i as f32 * angle).sin(),
            (i as f32 * angle).cos(),
        ));
    }
    let polygon: Polygon = Polygon::new(n_o_nodes, poly_nodes);
    polygon
}

pub fn steepest_ascent(
    points: &[Point],
    mut polygon: Polygon,
    stepsize: f32,
    circumference: &mut Vec<f32>,
) -> Polygon {
    let mut rnd = rand::rng();
    let mut actual_circumference = 0_f32;
    let n_o_nodes = polygon.nodes.len();
    if circumference.is_empty() {
        for i in 0..n_o_nodes {
            actual_circumference +=
                calculate_line_length(&polygon.nodes[i], &polygon.nodes[(i + 1) % n_o_nodes]);
        }
        circumference.push(actual_circumference);
    }
    for i in 0..n_o_nodes {
        let original_point = polygon.nodes[i];
        let new_x_diff = rnd.random_range(-stepsize..=stepsize);
        let new_y_diff = rnd.random_range(-stepsize..=stepsize);
        polygon.nodes[i] = Point::new(original_point.x - new_x_diff, original_point.y - new_y_diff);
        if !points_are_in_bounds(points, &polygon) {
            polygon.nodes[i] = original_point;
        }
        actual_circumference = 0_f32;
        for i in 0..n_o_nodes {
            actual_circumference +=
                calculate_line_length(&polygon.nodes[i], &polygon.nodes[(i + 1) % n_o_nodes]);
        }
        if actual_circumference > *circumference.last().unwrap() {
            polygon.nodes[i] = original_point;
        }
    }
    circumference.push(actual_circumference);
    polygon
}

fn calculate_line_length(point1: &Point, point2: &Point) -> f32 {
    ((point1.x - point2.x).abs().powi(2) + (point1.y - point2.y).abs().powi(2)).sqrt()
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
