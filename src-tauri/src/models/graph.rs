use lombok::{AllArgsConstructor, Getter};
use uuid::Uuid;

use crate::dto::api::{GraphDisplayProperties, GraphDisplayStyle};

use super::structure::Structure;


#[derive(AllArgsConstructor, Getter, Debug)]
pub struct Graph {
    id: Uuid,
    structures: Vec<Structure>,
    graph_display_properties: GraphDisplayProperties,
    graph_style: GraphDisplayStyle
}