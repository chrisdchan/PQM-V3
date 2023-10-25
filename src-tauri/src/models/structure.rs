use anyhow::{anyhow, Result};
use lombok::{AllArgsConstructor, Builder, Getter, Setter};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::api::{StructureDisplayProperties, StructureDisplayStyle};

use super::spline::Spline;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Metric {
    CurrentDensity,
    ElectricField,
    SpecificAbsorbanceRate,
}

#[derive(AllArgsConstructor, Getter, Setter, Clone, Debug)]
pub struct Structure {
    id: Uuid,
    name: String,
    file_name: String,
    metric: Metric,
    splines: Vec<Spline>,
    display_properties: StructureDisplayProperties,
    style: StructureDisplayStyle,
}

impl Structure {
    pub fn get_y(&self, x: f32) -> Result<f32> {
        let spline: &Spline = self.get_spline_from_x(x)?;
        spline.get_y(x)
    }

    fn get_spline_from_x(&self, x: f32) -> Result<&Spline> {
        self.verify_in_domain(x)?;
        let n = self.splines.len();
        let mut mid_ind = n / 2;
        let mut start = 0;
        let mut end = n;

        let mut spline = self
            .splines
            .get(mid_ind)
            .ok_or(anyhow!("Index out of bounds"))?;
        let mut count = 0;
        loop {
            count += 1;

            match spline.compare_with_domain(x) {
                std::cmp::Ordering::Equal => return Ok(spline),
                std::cmp::Ordering::Less => end = mid_ind,
                std::cmp::Ordering::Greater => start = mid_ind,
            }
            mid_ind = (start + end) / 2;
            spline = self.splines.get(mid_ind).ok_or(anyhow!(
                "Index {} out of bounds for [{}, {}]",
                mid_ind,
                start,
                end
            ))?;

            if count > 50 {
                panic!("Infinite Loop occurred for mid_index = {}", mid_ind);
            }
        }
    }

    fn verify_in_domain(&self, x: f32) -> Result<bool> {
        let first_spline: &Spline = self
            .splines
            .first()
            .ok_or(anyhow!("Splines for structure is an empty list"))?;
        let last_spline: &Spline = self
            .splines
            .last()
            .ok_or(anyhow!("Splines for structure is an empty list"))?;

        let res = first_spline.get_x1() < &x && &x < last_spline.get_x2();
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_structure() -> Structure {
        let (a, b, c, d) = (5., 5., 5., 5.);
        let (y1, y2) = (100., 100.);
        let splines = vec![
            Spline::new(0., y1, 4., y2, a, b, c, d),
            Spline::new(4., y1, 5., y2, a, b, c, d),
            Spline::new(5., y1, 6.7, y2, a, b, c, d),
            Spline::new(6.7, y1, 10., y2, a, b, c, d),
            Spline::new(10., y1, 14., y2, a, b, c, d),
        ];
        Structure::new(
            Uuid::new_v4(),
            "name".to_owned(),
            "file name".to_owned(),
            Metric::CurrentDensity,
            splines.clone(),
            StructureDisplayProperties::default(),
            StructureDisplayStyle::default(),
        )
    }

    #[test]
    fn test_get_spline_from_x() {
        // arrange
        let structure = get_structure();
        // act
        let spline = structure.get_spline_from_x(0.5).unwrap();

        assert_eq!(structure.get_splines().get(0).unwrap(), spline);
    }
}