use lombok::{AllArgsConstructor, Getter};

use super::structure::Structure;


#[derive(AllArgsConstructor, Getter, Debug)]
pub struct Graph {
    structures: Vec<Structure>
}