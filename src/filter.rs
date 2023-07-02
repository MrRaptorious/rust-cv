use std::error::Error;
use rayon::prelude::*;

// nxn matrix to apply on a picture
pub struct Kernel {
    val: Vec<f32>,
    size: usize,
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
#[allow(dead_code)]
pub fn to_gray(img: &mut [u8]) {
    img.par_chunks_mut(3).for_each(|pxl| match pxl {
        [r, g, b] => {
            let gray = (((*r as u16) + (*g as u16) + (*b as u16)) / 3) as u8;
            *r = gray;
            *g = gray;
            *b = gray;
        }
        _ => unreachable!(),
    });
}