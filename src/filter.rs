use std::error::Error;
use rayon::prelude::*;

// nxn matrix to apply on a picture
pub struct Kernel {
    val: Vec<f32>,
    size: usize,
}

impl Kernel{
    pub fn print(&self) {
        self.val.chunks(self.size).for_each(|row| println!("{:?}", row));
    }
}

// Applies the kernel to the image returning a new image
pub fn apply_kernel(
    img: &[u8],
    img_size: (u32,u32),
    kernel: &Kernel,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let kernel_ancor_x = (kernel.size / 2) as i32;
    let kernel_ancor_y = (kernel.size / 2) as i32;

    let mut filterd_image = vec![0; img.len()];
    filterd_image.copy_from_slice(img);

    // got through each pixel in result image
    filterd_image
        .chunks_exact_mut(3)
        .enumerate()
        .for_each(|(i, rgb)| {

            // get pixel index of current iteration
            let img_x = i as u32 % img_size.0;
            let img_y = if i > 0 { i as u32 / img_size.0 } else { 0 };

            let mut kernel_result: Vec<f32> = vec![0f32, 0f32, 0f32];

            // go through kernel
            kernel.val.iter().enumerate().for_each(|(j, kernel_val)| {
                // note: coord 0 is on the ancor!
                let kernel_x = (j as u32 % kernel.size as u32) as i32 - kernel_ancor_x;
                let kernel_y = if j > 0 {
                    (j as i32 / kernel.size as i32) - kernel_ancor_y
                } else {
                    0 - kernel_ancor_y
                };

                let kx_in_image = img_x as i32 + kernel_x;
                let ky_in_image = img_y as i32 + kernel_y;

                // check if kernel part not out of bounds
                if !((kx_in_image < 0 || kx_in_image >= img_size.0 as i32)
                    || (ky_in_image < 0 || ky_in_image >= img_size.1 as i32))
                {
                    let current_pos_in_image =
                        kx_in_image * 3 + ky_in_image * 3 * img_size.0 as i32;

                    kernel_result[0] += kernel_val * img[current_pos_in_image as usize] as f32;
                    kernel_result[1] += kernel_val * img[current_pos_in_image as usize + 1]as f32;
                    kernel_result[2] += kernel_val * img[current_pos_in_image as usize + 2]as f32;
                }
            });

            rgb[0] = kernel_result[0].clamp(0f32, 255f32) as u8;
            rgb[1] = kernel_result[1].clamp(0f32, 255f32) as u8;
            rgb[2] = kernel_result[2].clamp(0f32, 255f32) as u8;
        });

    Ok(filterd_image)
}

// Converts the image to a black and white version by averaging the R, G and B channels
pub fn to_gray(img: &[u8]) -> Vec<u8> {

    let mut gray_img = vec![0; img.len()];
    gray_img.copy_from_slice(img);

    gray_img.par_chunks_mut(3).for_each(|pxl| match pxl {
        [r, g, b] => {
            let gray = (((*r as u16) + (*g as u16) + (*b as u16)) / 3) as u8;
            *r = gray;
            *g = gray;
            *b = gray;
        }
        _ => unreachable!(),
    });

    gray_img
}

// Get a default Gaussian 5x5 Kernel
pub fn get_gaussian_kernel() -> Kernel {
    Kernel {
        val: vec![
            0.0037, 0.0147, 0.0256, 0.0147, 0.0037,
            0.0147, 0.0586, 0.0952, 0.0586, 0.0147,
            0.0256, 0.0952, 0.1502, 0.0952, 0.0256,
            0.0147, 0.0586, 0.0952, 0.0586, 0.0147,
            0.0037, 0.0147, 0.0256, 0.0147, 0.0037
        ],
        size: 5,
    }
}

// Get a default outline 3x3 Kernel
pub fn get_outline_kernel() -> Kernel {
    Kernel {
        val: vec![
            -1.0, -1.0, -1.0,
            -1.0, 8.0, -1.0,
            -1.0, -1.0, -1.0,
        ],
        size: 3,
    }
}

// Get a default right sobel 3x3 Kernel
pub fn get_right_sobel_kernel() -> Kernel {
    Kernel {
        val: vec![
            -1.0, 0.0, 1.0,
            -2.0, 0.0, 2.0,
            -1.0, 0.0, 1.0,
        ],
        size: 3,
    }
}

// Get a default bottom sobel 3x3 Kernel
pub fn get_bottom_sobel_kernel() -> Kernel {
    Kernel {
        val: vec![
            -1.0, -2.0, -1.0,
            0.0, 0.0, 0.0,
            1.0, 2.0, 1.0,
        ],
        size: 3,
    }
}

// Get a default sharpening 3x3 Kernel with given strenght
pub fn get_sharpening_kernel(strength: f32) -> Kernel {
    Kernel {
        val: vec![
            0.0, (-1.0/4.0) * strength , 0.0,
            (-1.0/4.0) * strength, ((1.0) * strength) + 1.0f32, (-1.0/4.0) * strength,
            0.0, (-1.0/4.0) * strength, 0.0
        ],
        size: 3,
    }
}

// Get a default sharpening 3x3 Kernel with strength 1.0
#[allow(unused_macros)]
macro_rules! sharpening_kernel {
    ($strength: expr) => {
        filter::get_sharpening_kernel($strength)
    };
    () => {
        filter::get_sharpening_kernel(1.0f32)
    };
}

#[allow(unused_imports)]
pub(crate) use sharpening_kernel;
