use lombok::AllArgsConstructor;
use serde::Serialize;

use crate::models::styles::structure_display_properties::LineType;


#[derive(Serialize, AllArgsConstructor, Clone)]
pub struct Curve {
    lines: Vec<Line>,
    line_type: LineType,
}

#[derive(Serialize, AllArgsConstructor, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

#[derive(Serialize, AllArgsConstructor, Clone)]
pub struct Point {
    x: f32,
    y: f32,
}
