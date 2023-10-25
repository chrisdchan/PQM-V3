use super::api::{
    AxisDisplayProperties, AxisStyle, GraphDisplayProperties, GraphDisplayStyle, LabelStyle,
    LineStyle, LineType, Margin, StructureDisplayProperties, StructureDisplayStyle, TickLineStyle,
};

impl Default for GraphDisplayProperties {
    fn default() -> Self {
        let title_name = "".to_owned();
        let x_axis_display_properties = AxisDisplayProperties::default();
        let y_axis_display_properties = AxisDisplayProperties::default();
        Self::new(
            title_name,
            x_axis_display_properties,
            y_axis_display_properties,
        )
    }
}

impl Default for AxisDisplayProperties {
    fn default() -> Self {
        let name = "".to_owned();
        let start = 0.;
        let end = 100.;
        let num_ticks = 5;
        let tick_gap = 10.;
        let percision = 0;
        Self::new(name, start, end, num_ticks, tick_gap, percision)
    }
}

impl Default for GraphDisplayStyle {
    fn default() -> Self {
        let outer_color = "0xffffff".to_string();
        let inner_color = "0x000000".to_string();
        let margin = Margin::default();
        let title_style = LabelStyle::default();
        let x_axis_style = AxisStyle::default();
        let y_axis_style = AxisStyle::default();
        Self::new(
            outer_color,
            inner_color,
            margin,
            title_style,
            x_axis_style,
            y_axis_style,
        )
    }
}

impl Default for StructureDisplayProperties {
    fn default() -> Self {
        let line_type = LineType::SOLID;
        let resolution = 100;
        Self::new(line_type, resolution)
    }
}

impl Default for StructureDisplayStyle {
    fn default() -> Self {
        let color = "0xff00ff".to_string();
        StructureDisplayStyle::new(color)
    }
}

impl Default for Margin {
    fn default() -> Self {
        let left = 0.2;
        let right = 0.2;
        let top = 0.2;
        let bottom = 0.2;
        Self::new(left, right, top, bottom)
    }
}

impl Default for LabelStyle {
    fn default() -> Self {
        let text_color = "0x000000".to_string();
        let font_size = 12;
        let x_offset = 0.0;
        let y_offset = 0.0;
        Self::new(text_color, font_size, x_offset, y_offset)
    }
}

impl Default for AxisStyle {
    fn default() -> Self {
        let line_style = LineStyle::default();
        let title_style = LabelStyle::default();
        let tick_line_style = TickLineStyle::default();
        let tick_label_style = LabelStyle::default();
        Self::new(line_style, title_style, tick_line_style, tick_label_style)
    }
}

impl Default for LineStyle {
    fn default() -> Self {
        let width = 3.0;
        let color = "0x000000".to_string();
        Self::new(width, color)
    }
}

impl Default for TickLineStyle {
    fn default() -> Self {
        let line_style = LineStyle::default();
        let length = 3.0;
        Self::new(line_style, length)
    }
}
