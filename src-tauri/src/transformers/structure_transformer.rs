use anyhow::Result;

use crate::{
    dto::{
        input::structure_properties::{LineType, StructureDisplayProperties},
        response::{
            curve::{Curve, Line, Point},
            structure_response::StructureResponse,
        },
    },
    models::core::structure::Structure,
};

pub fn to_structure_response(
    structure: &Structure,
    display_properties: &StructureDisplayProperties,
) -> Result<StructureResponse> {
    let dx: f32 = (display_properties.get_end() - display_properties.get_start())
        / *display_properties.get_resolution() as f32;

    let mut x1 = display_properties.get_end().to_owned();
    let mut x2 = x1 + dx;
    let mut y1;
    let mut y2;

    let mut lines: Vec<Line> = Vec::new();

    for _i in 0..*display_properties.get_resolution() {
        y1 = structure.get_y(x1)?;
        y2 = structure.get_y(x2)?;
        let point1 = Point::new(x1, y1);
        let point2 = Point::new(x2, y2);

        lines.push(Line::new(point1, point2));

        x1 = x2;
        x2 = x2 + dx;
    }

    let curve = Curve::new(lines, LineType::Solid);
    let structure_response = StructureResponse::new(structure.get_file_name().to_string(), curve);
    Ok(structure_response)
}
