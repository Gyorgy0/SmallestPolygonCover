use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Serialize, Deserialize)]
pub struct Point {
    pub id: u64,
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PolygonNode {
    id: u64,
    x: f32,
    y: f32,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct PolygonLine {
    point1: PolygonNode,
    point2: PolygonNode,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Polygon {
    pub k: u64,                  // Number of polygon nodes - körbeírandó poligon fokszáma
    pub nodes: Vec<PolygonNode>, // Polygon nodes - poligon csúcsai
    pub lines: Vec<PolygonLine>, // Polygon lines - poligon határvonalai
}

pub fn setupPoints(n_o_points: u8) -> Vec<Point> {
    let mut points = vec![Point::default(); n_o_points as usize];
    let mut rnd = rand::rng();
    for i in 0..points.len() {
        // Ensures that they're inside the polygon x^2+y^2=r^2
        let mut x = rnd.random_range(-1_f32..=1_f32);
        let mut y = rnd.random_range(-1_f32..=1_f32);
        if (x.powi(2) >= 1.0 - y.powi(2)) {
            let radius = 1.0 - y.powi(2);
            x = rnd.random_range(-radius..radius);
            x = x.sqrt();
        }
        else if (y.powi(2) >= 1.0 - x.powi(2)) {
            let radius = 1.0 - x.powi(2);
            y = rnd.random_range(-radius..radius);
            y = y.sqrt();
        }
        points[i].x = x;
        points[i].y = y;
    }
    points
}

pub fn hill_climbing() {}
