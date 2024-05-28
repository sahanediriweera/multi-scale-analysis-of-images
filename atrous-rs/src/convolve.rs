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