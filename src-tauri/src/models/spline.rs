use std::cmp::Ordering;

use anyhow::anyhow;
use anyhow::Result;
use lombok::{AllArgsConstructor, Builder, Getter, Setter};
use roots::{find_roots_cubic, Roots};
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

    pub fn get_x(&self, y: f32) -> Result<f32> {
        if y > self.y1 || self.y2 > y {
            return Err(anyhow!("Input {} out of bounds on spline {:?}", y, self));
        }
        let roots = find_roots_cubic(self.a, self.b, self.c, self.d - y);
        let roots = match roots {
            Roots::No(arr) => arr.to_vec(),
            Roots::One(arr) => arr.to_vec(),
            Roots::Two(arr) => arr.to_vec(),
            Roots::Three(arr) => arr.to_vec(),
            Roots::Four(arr) => arr.to_vec(),
        };

        let roots: Vec<f32> = roots
            .into_iter()
            .filter(|root| self.in_domain(*root))
            .collect();

        if roots.len() == 1 {
            Ok(roots[0])
        } else {
            println!("{:?}", roots);
            Err(anyhow!(
                "There should be exactly one root in the spline domain"
            ))
        }
    }

    pub fn in_domain(&self, x: f32) -> bool {
        println!("{:?} <= {:?} < {:?}", self.x1, x, self.x2);
        self.x1 <= x && x < self.x2
    }

    pub fn compare_with_domain(&self, x: f32) -> Ordering {
        if x < self.x1 {
            Ordering::Less
        } else if self.x2 < x {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
    pub fn compare_with_range(&self, y: f32) -> Ordering {
        if y > self.y1 {
            Ordering::Less
        } else if y < self.y2 {
            Ordering::Greater
        } else {
            Ordering::Equal
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
