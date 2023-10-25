use std::cmp::Ordering;

use anyhow::anyhow;
use anyhow::Result;
use lombok::{AllArgsConstructor, Builder, Getter, Setter};
use serde::{Deserialize, Serialize};

#[derive(
    Serialize, PartialEq, Deserialize, Debug, Getter, Setter, AllArgsConstructor, Builder, Clone,
)]
pub struct Spline {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,

    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

impl Spline {
    pub fn get_y(&self, x: f32) -> Result<f32> {
        if x < self.x1 || self.x2 < x {
            return Err(anyhow!("Input {} out of bounds on spline {:?}", x, self));
        }
        let y = self.a * x.powi(3) + self.b * x.powi(2) + self.c * x + self.d;
        Ok(y)
    }

    pub fn in_domain(&self, x: f32) -> bool {
        self.x1 <= x && x < self.x2
    }

    pub fn compare_with_domain(&self, x: f32) -> Ordering {
        if x < self.x1 {
            return Ordering::Less;
        } else if self.x2 < x {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_y_invalid_input() {
        let spline = Spline::new(5.0, 90.0, 6.0, 50.0, 1.0, 1.0, 1.0, 1.0);
        let res = spline.get_y(4.5);
        assert!(res.is_err())
    }
}
