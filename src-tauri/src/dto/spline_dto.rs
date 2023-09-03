use lombok::AllArgsConstructor;

#[derive(AllArgsConstructor, Debug, Clone)]
pub struct SplineDtoRaw {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

#[derive(Clone, Debug)]
pub struct SplineDtoWithDeriv {
    pub delta: f32,
    pub left_deriv: f32,
    pub right_deriv: f32,
}
