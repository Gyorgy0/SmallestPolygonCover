use std::f32::consts::PI;

use egui::Pos2;
use rand::Rng;
use serde::{Deserialize, Serialize};

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

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct PolygonLine {
    pub point1: Point,
    pub point2: Point,
}

impl PolygonLine {
    pub fn new(point1: Point, point2: Point) -> Self {
        Self {
            point1: point1,
            point2: point2,
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Polygon {
    pub k: u8,                   // Number of polygon nodes - körbeírandó poligon fokszáma
    pub nodes: Vec<Point>,       // Polygon nodes - poligon csúcsai
    pub lines: Vec<PolygonLine>, // Polygon lines - poligon határvonalai
}

impl Polygon {
    pub fn new(k: u8, nodes: Vec<Point>, lines: Vec<PolygonLine>) -> Self {
        Self {
            k: k,
            nodes: nodes,
            lines: lines,
        }
    }
}

pub fn setupPoints(n_o_points: u8) -> Vec<Point> {
    let mut points = vec![Point::default(); n_o_points as usize];
    let mut rnd = rand::rng();
    for i in 0..points.len() {
        // Ensures that the generated points are inside the circular area (x^2+y^2=r^2)
        // Biztosítja, hogy a pont ne kerüljön a körön kívüli területre az x^2+y^2=r^2 összefüggés segítségével
        let mut x = rnd.random_range(-1_f32..=1_f32);
        let mut y = rnd.random_range(-1_f32..=1_f32);
        if (x.powi(2) > 1.0 - y.powi(2)) {
            let radius = 1.0 - y.powi(2);
            x = rnd.random_range(-radius..radius);
        } else if (y.powi(2) > 1.0 - x.powi(2)) {
            let radius = 1.0 - x.powi(2);
            y = rnd.random_range(-radius..radius);
        }
        points[i].x = x;
        points[i].y = y;
    }
    points
}

pub fn setupPolygon(n_o_nodes: u8) -> Polygon {
    let mut poly_nodes: Vec<Point> = vec![Point::default(); n_o_nodes as usize];
    let mut poly_lines: Vec<PolygonLine> = vec![PolygonLine::default(); n_o_nodes as usize];
    let angle = (2_f32 * PI) / n_o_nodes as f32;
    for i in 0..n_o_nodes as usize {
        poly_nodes[i] = Point::new((i as f32 * angle).sin(), (i as f32 * angle).cos());
    }
    for i in 0..n_o_nodes as usize {
        poly_lines[i].point1 = poly_nodes[i % n_o_nodes as usize];
        poly_lines[i].point2 = poly_nodes[(i + 1) % n_o_nodes as usize];
    }
    let polygon: Polygon = Polygon::new(n_o_nodes, poly_nodes, poly_lines);
    polygon
}

pub fn steepest_ascent() {
    
}
