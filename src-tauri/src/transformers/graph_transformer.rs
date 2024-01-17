use crate::{
    dto::api::{GraphDisplay, GraphDisplayStyle, StructureDisplay},
    models::graph::Graph,
};
use std::sync::Arc;

use crate::dto::api::GraphDisplayProperties;
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
                Arc::clone(structure),
                *x_axis_display_props.get_start(),
                *x_axis_display_props.get_end(),
            )
        })
        .collect::<Result<Vec<StructureDisplay>>>()?;

    let graph_display = GraphDisplay::builder()
        .id(graph.get_id().to_string())
        .graph_type(graph.get_graph_type().clone())
        .structures(structure_displays)
        .display_properties(graph.get_graph_display_properties().clone())
        .build();

    Ok(graph_display)
}
