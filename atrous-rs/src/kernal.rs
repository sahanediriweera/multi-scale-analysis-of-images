#[derive(Copy,Clone)]
pub struct LinearInterpolationKernal{
    values:[[f2;3];3]
}

impl Default for LinearInterpolationKernal{
    fn default()->Self {
        Self {
            values:[
                [1. / 16., 1. / 8., 1. / 16.],
                [1. / 8., 1. / 4., 1. / 8.],
                [1. / 16., 1. / 8., 1. / 16.],
            ]
        }
    }
}
