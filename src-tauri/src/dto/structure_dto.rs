use lombok::{AllArgsConstructor};

use crate::models::core::structure::Metric;

use super::spline_dto::SplineDtoRaw;

#[derive(AllArgsConstructor, Debug)]
pub struct StructureDto {
    pub name: String,
    pub file_name: String,
    pub frequency: f32,
    pub metric: Metric,
    pub splines: Vec<SplineDtoRaw>,
}
