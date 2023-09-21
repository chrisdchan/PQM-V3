use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct AxisDisplayProperties {
    name: String,
    start: f32,
    end: f32,
    num_ticks: i32,
    tick_gap: f32,
    percision: i32,
}


impl Default for AxisDisplayProperties {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            start: 0.,
            end: 100.,
            num_ticks: 5,
            tick_gap: 10.,
            percision: 0
        }
    }
}