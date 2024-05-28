use ndarray::Array2;

pub struct ATrousTransform{
    input: Array2<f32>,
    levels:usize,
    current_level:usize,
    width:usize,
    height:usize
}

use image::GenericImageView;

impl ATrousTransform {
    pub fn new(input: &image::DynamicImage,levels: usize){
        let (width,height) = input.dimensions();
        let (width,height) = (width as usize, height as usize);

        let mut data = Array2::<f32>::zeros((height,width));

        for(x,y,pixel) in input.to_luma32f().enumerate_pixels(){
            data[[y as usize,x as usize]] = pixel.0[0];


        }

        Self{
            input:data,
            levels,
            current_level:0,
            width,
            height
        }
    }
}