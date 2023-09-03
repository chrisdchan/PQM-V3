use lombok::Getter;
use serde::Serialize;

#[derive(Debug, Getter)]
pub struct StructureDisplayProperties {
    line_type: LineType,
    start: f32,
    end: f32,
    resolution: i32,
}

#[derive(Debug, Serialize, Clone)]
pub enum LineType {
    Solid,
    Dotted,
    Dashed,
}

impl Default for StructureDisplayProperties {
    fn default() -> Self {
        Self {
            line_type: LineType::Solid,
            start: 0.,
            end: 100.,
            resolution: 100,
        }
    }
}
