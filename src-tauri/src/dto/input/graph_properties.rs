use serde::Deserialize;

use super::axis_properties::AxisProperties;

#[derive(Deserialize)]
pub struct GraphProperties {
    title_name: String,
    x_axis_properties: AxisProperties,
    y_axis_properties: AxisProperties,
}
