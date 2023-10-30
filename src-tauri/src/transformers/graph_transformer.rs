use crate::{
    dto::api::{GraphDisplay, StructureDisplay},
    models::graph::Graph,
};

use anyhow::Result;

use super::structure_transformer;

pub fn to_graph_display(graph: &Graph) -> Result<GraphDisplay> {
    let x_axis_display_props = graph
        .get_graph_display_properties()
        .get_x_axis_display_properties();

    let structure_displays: Vec<StructureDisplay> = graph
        .get_structures()
        .into_iter()
        .map(|(id, structure)| {
            structure_transformer::to_structure_display(
                structure,
                *x_axis_display_props.get_start(),
                *x_axis_display_props.get_end(),
            )
        })
        .collect::<Result<Vec<StructureDisplay>>>()?;

    let graph_display = GraphDisplay::new(
        graph.get_id().to_string(),
        graph.get_graph_type().clone(),
        structure_displays,
        graph.get_graph_display_properties().clone(),
        graph.get_graph_style().clone(),
    );

    Ok(graph_display)
}
