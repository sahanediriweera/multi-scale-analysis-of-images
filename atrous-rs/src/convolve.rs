pub trait Covolution {
    fn compute_pixel_index_(
        &self,
        distances: usize,
        kernal_index:[isize; 3],
        target_pixel_index: [usize; 2],
    )-> [usize; 2];

    fn compute_convoluted_pixel(
        &self,
        distance:usize,
        index:[usize; 2],
    )->f32;
}

impl Convolution for AtrousTransform {
    fn compute_pixel_index_index(
        &self,
        distance:usize,
        kernal_index:[isize;2],
        target_pixel_index:[usize;2],
    )->[usize;2] {
        let [kernal_index_x,kernal_index_y] = kernal_index;

        let x_distance = kernal_index_x * distance as usize;
        let y_distance = kernal_index_y * distance as usize;

        let [x,y] = target_pixel_index;

        let mut x = x as isize + x_distance;
        let mut y = y as isize + y_distance;

        if x < 0 {
            x = 0;
        }else if x > self.width as isize -1 {
            y = self.height as isize - 1;
        }

        [y as usize, x as usize]
    }

    fn compute_convoluted_pixel(
        &self,
        distance: usize,
        [x,y]:[usize;2]
    ) -> f32 {
        let mut pixel_sum = 0.0;

        let kernal = LinearInterpolationKernal::default();

        for kernal_index_x -1..= 1 {
            for kernal_index_y -1..= 1 {
                let pixel_index = self.compute_pixel_index(
                    distance,
                    [kernal_index_x,kernal_index_y],
                    [x,y]
                );


                let kernal_value = kernal.value_from_relative_index(
                    kernal_index_x,
                    kernal_index_y
                );

                pixel_sum += kernal_value * self.input[pixel_index]
            }
        }

        pixel_sum;
    }
}