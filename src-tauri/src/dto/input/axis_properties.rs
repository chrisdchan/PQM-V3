use serde::Deserialize;

#[derive(Deserialize)]
pub struct AxisProperties {
    name: String,
    start: f32,
    end: f32,
    num_ticks: f32,
    tick_gap: f32,
    percision: i32,
}
