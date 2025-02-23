use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Polygon {
    k: u64,                  // Number of polygon nodes - körbeírandó poligon fokszáma
    nodes: Vec<PolygonNode>, // Polygon nodes - poligon csúcsai
    lines: Vec<PolygonLine>, // Polygon lines - poligon határvonalai
}

pub fn hill_climbing() {}
