use std::collections::HashMap;
use std::sync::Arc;
use lombok::{AllArgsConstructor, Getter};
use uuid::Uuid;

use crate::dto::api::{GraphDisplayProperties, GraphDisplayStyle, GraphType};

use super::structure::Structure;


#[derive(AllArgsConstructor, Getter, Debug)]
pub struct Graph {
    id: Uuid,
    graph_type: GraphType,
    structures: HashMap<Uuid, Arc<Structure>>,
    graph_display_properties: GraphDisplayProperties,
    graph_style: GraphDisplayStyle
}