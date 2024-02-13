use anyhow::{ensure, Result};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::dto::api::StructureTableDisplay;
use crate::{
    dto::api::{Curve, Line, Point, StructureDisplay, StructureDisplayProperties},
    models::structure::Structure,
};

pub fn to_structure_display(
    structure_mutex: Arc<Mutex<Structure>>,
    start: f32,
    end: f32,
) -> Result<StructureDisplay> {
    let structure = structure_mutex.lock().unwrap();
    let display_properties: &StructureDisplayProperties = structure.get_display_properties();

    let resolution: i32 = *display_properties.get_resolution();
    ensure!(resolution > 0, "Graph Resolution must be greater than 0");
    let dx: f32 = (end - start) / *display_properties.get_resolution() as f32;
    // let dx: f32 = (end - start) / *display_properties.get_resolution() as f32;

    let mut x1 = start;
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

    let curve = Curve::new(lines);
    let structure_display = StructureDisplay::builder()
        .id(structure.get_id().to_string())
        .curve(curve)
        .display_properties(structure.get_display_properties().clone())
        .build();

    Ok(structure_display)
}

pub fn to_structure_table(structure_mutex: Arc<Mutex<Structure>>) -> Result<StructureTableDisplay> {
    let structure = structure_mutex.lock().unwrap();
    let structure_name = structure.get_name();
    let volume = -1.;
    let cc = structure.get_y(0.03)? * volume;

    let y_values: Vec<f32> = vec![100., 95., 90., 50., 5.];
    let x_values: Vec<f32> = y_values
        .iter()
        .map(|y| structure.get_x(*y))
        .collect::<Result<Vec<f32>>>()?;

    let map: HashMap<String, f32> = y_values
        .iter()
        .map(|y| y.to_string())
        .zip(x_values.into_iter())
        .collect();

    let structure_table = StructureTableDisplay::new(structure_name.clone(), volume, map, cc);
    Ok(structure_table)
}
