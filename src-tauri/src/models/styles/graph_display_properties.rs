use serde::{Deserialize, Serialize};

use super::axis_display_properties::AxisDisplayProperties;

#[derive(Serialize, Deserialize, Clone)]
pub struct GraphDisplayProperties {
    title_name: String,
    x_axis_properties: AxisDisplayProperties,
    y_axis_properties: AxisDisplayProperties,
}

impl Default for GraphDisplayProperties {
    fn default() -> Self {
        Self {
            title_name: "".to_owned(),
            x_axis_properties: AxisDisplayProperties::default(),
            y_axis_properties: AxisDisplayProperties::default(),
        }
    }
}
