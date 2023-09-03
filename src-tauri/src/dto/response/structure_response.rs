use lombok::AllArgsConstructor;
use serde::Serialize;

use super::curve::Curve;

#[derive(Serialize, AllArgsConstructor, Clone)]
pub struct StructureResponse {
    name: String,
    curve: Curve,
}
