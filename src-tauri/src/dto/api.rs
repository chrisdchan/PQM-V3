use std::collections::HashMap;
use lombok::{AllArgsConstructor, Getter};
use serde::Serialize;
#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ModelDisplay {
    id: String,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphDisplay {
    id: String,
    graph_type: GraphType,
    structures: Vec<StructureDisplay>,
    graph_display_properties: GraphDisplayProperties,
    graph_display_style: GraphDisplayStyle,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GraphType {
    CurrentDensity,
    EField,
    SAR
}


#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructureDisplay {
    id: String,
    curve: Curve,
    structure_display_properties: StructureDisplayProperties,
    structure_display_style: StructureDisplayStyle,
}

#[derive(Serialize, Debug, AllArgsConstructor, Getter, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphDisplayProperties {
    title_name: String,
    x_axis_display_properties: AxisDisplayProperties,
    y_axis_display_properties: AxisDisplayProperties,
}

#[derive(Serialize, Debug, AllArgsConstructor, Getter, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructureDisplayProperties {
    line_type: LineType,
    resolution: i32,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum LineType {
    Solid,
    Dotted,
    Dashed,
}

#[derive(Serialize, Debug, AllArgsConstructor, Getter, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisDisplayProperties {
    name: String,
    start: f32,
    end: f32,
    num_ticks: i32,
    tick_gap: f32,
    percision: i32,
}

// Attributes that are aesthetic only and require no backend logic
#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphDisplayStyle {
    outer_color: String,
    inner_color: String,
    margin: Margin,
    title_style: LabelStyle,
    x_axis_style: AxisStyle,
    y_axis_style: AxisStyle,
}

// Attributes that are aesthetic only and require no backend logic
#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructureDisplayStyle {
    color: String,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Margin {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LabelStyle {
    text_color: String,
    font_size: i32,
    x_offset: f32,
    y_offset: f32,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AxisStyle {
    line_style: LineStyle,
    title_style: LabelStyle,
    tick_line_style: TickLineStyle,
    tick_label_style: LabelStyle,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LineStyle {
    width: f32,
    color: String,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TickLineStyle {
    line_style: LineStyle,
    length: f32,
}
#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Curve {
    lines: Vec<Line>,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    start: Point,
    end: Point,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StructureTableDisplay{
    structure_name: String,
    volume: f32,
    map: HashMap<String, f32>,
    cc: f32,
}
#[derive(Serialize, Debug, AllArgsConstructor, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GraphTableDisplay{
    structure_table_displays: Vec<StructureTableDisplay>
}
