use std::error::Error;
use rayon::prelude::*;
use crate::image;

// nxn matrix to apply on a picture
pub struct Kernel {
    pub val: Vec<f32>,
    pub size: usize,
}

impl Kernel{
    pub fn print(&self) {
        self.val.chunks(self.size).for_each(|row| println!("{:?}", row));
    }
}

// Creates a copy of the image, applies the kernel and saves the result in the copied image
pub fn apply_kernel(
    img: &image::Image,
    kernel: &Kernel,
) -> Result<image::Image, Box<dyn Error>> {
    // anchor always in the middle
    let kernel_anchor_x = (kernel.size / 2) as i32;
    let kernel_anchor_y = (kernel.size / 2) as i32;

    let mut filterd_image = vec![0; img.data.len()];
    let pixel_width = img.get_pixl_width();

    // got through each pixel in result image
    filterd_image
        .par_chunks_exact_mut(pixel_width)
        .enumerate()
        .for_each(|(i, pixel)| {

            // get pixel index of current iteration
            let img_x = i as u32 % img.width as u32;
            let img_y = if i > 0 { i as u32 / img.width as u32 } else { 0 };

            let mut kernel_result: Vec<f32> = vec![0f32; pixel_width];

            // go through kernel
            kernel.val.iter().enumerate().for_each(|(j, kernel_val)| {
                // coord 0|0 is on the anchor!
                let kernel_x = (j as u32 % kernel.size as u32) as i32 - kernel_anchor_x;
                let kernel_y = if j > 0 {
                    (j as i32 / kernel.size as i32) - kernel_anchor_y
                } else {
                    0 - kernel_anchor_y
                };

                let kx_in_image = img_x as i32 + kernel_x;
                let ky_in_image = img_y as i32 + kernel_y;

                // check if kernel part not out of bounds
                if !((kx_in_image < 0 || kx_in_image >= img.width as i32)
                    || (ky_in_image < 0 || ky_in_image >= img.height as i32))
                {
                    let current_pos_in_image =
                        kx_in_image * pixel_width as i32 + ky_in_image * pixel_width as i32 * img.width as i32;

                    kernel_result.iter_mut().enumerate().for_each(|(i,val)|{
                        *val += kernel_val * img.data[current_pos_in_image as usize + i] as f32;
                    });
                }
            });

            pixel.iter_mut().zip(kernel_result.iter()).for_each(|(px, rs)|{
                *px = rs.clamp(0f32, 255f32) as u8;
            });
        });

    Ok(image::Image{width:img.width, height:img.height, color_type: img.color_type, data:filterd_image})
}

