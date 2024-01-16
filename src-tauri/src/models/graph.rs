use std::collections::HashMap;
use std::sync::Arc;
use lombok::{AllArgsConstructor, Builder, Getter};
use uuid::Uuid;
use anyhow::Result;

use crate::dto::api::{GraphDisplayProperties, GraphDisplayStyle, GraphType};

use super::structure::Structure;


#[derive(AllArgsConstructor, Getter, Debug, Builder)]
pub struct Graph {
    id: Uuid,
    graph_type: Option<GraphType>,
    graph_display_properties: GraphDisplayProperties,
    structures: HashMap<Uuid, Arc<Structure>>,
}

impl Graph {
    pub fn get_domain(&self) -> Result<(f32, f32)> {
        self.structures
            .values()
            .into_iter()
            .try_fold((-f32::INFINITY, f32::INFINITY), |(acc_start, acc_end), structure| {
                let (start, end) = structure.get_domain()?;
                let new_start = if start > acc_start {start} else {acc_start};
                let new_end = if end < acc_end {end} else {acc_end};
                Ok((new_start, new_end))
            })
    }
}