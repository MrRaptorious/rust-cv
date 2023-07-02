use png::{BitDepth, ColorType, Decoder, OutputInfo};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // const IMG_PATH: &str = "res/lenna.png";
    // const OUTPUT_PATH: &str = "out/lenna.png";

    const IMG_PATH: &str = "res/test_small.png";
    const OUTPUT_PATH: &str = "out/test_small.png";

    let (buf, info) = load_image(IMG_PATH)?;

    let gaussian_kernel = get_gaussian_kernel();

    let filterd_image = apply_kernel(&buf, &info, &gaussian_kernel, (5, 5))?;

    save_image(OUTPUT_PATH, &filterd_image, info)?;

    Ok(())
}

fn get_gaussian_kernel() -> Vec<f32> {
    vec![
    0.0037, 0.0147, 0.0256, 0.0147, 0.0037,
    0.0147, 0.0586, 0.0952, 0.0586, 0.0147,
    0.0256, 0.0952, 0.1502, 0.0952, 0.0256,
    0.0147, 0.0586, 0.0952, 0.0586, 0.0147,
    0.0037, 0.0147, 0.0256, 0.0147, 0.0037
    ]
}

/// Load the png from the specified path
fn load_image(path: &str) -> Result<(Vec<u8>, OutputInfo), Box<dyn Error>> {
    let decoder = Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    // only allow certain pngs
    // have to have 3 channel with 8 bit depth
    if info.color_type != ColorType::Rgb || info.bit_depth != BitDepth::Eight {
        return Err("PNG format not supported, only 8 Bit depth and RGB images!"
            .to_string()
            .into());
    }

    Ok((buf, info))
}

/// save the image to the specified path
fn save_image(path: &str, data: &[u8], info: OutputInfo) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header()?;

    writer.write_image_data(data)?;

    Ok(())
}

#[allow(dead_code)]
fn to_gray(img: &mut [u8]) {
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

fn apply_kernel(
    img: &[u8],
    info: &OutputInfo,
    kernel: &[f32],
    kernel_info: (usize, usize),
) -> Result<Vec<u8>, Box<dyn Error>> {
    let kernel_ancor_x = (kernel_info.0 / 2) as i32;
    let kernel_ancor_y = (kernel_info.0 / 2) as i32;

    let mut filterd_image = vec![0; img.len()];
    filterd_image.copy_from_slice(img);

    // got through each pixel in result image
    filterd_image
        .chunks_exact_mut(3)
        .enumerate()
        .for_each(|(i, rgb)| {

            // get pixel index of current iteration
            let img_x = i as u32 % info.width;
            let img_y = if i > 0 { i as u32 / info.width } else { 0 };

            let mut kernel_result: Vec<f32> = vec![0f32, 0f32, 0f32];

            // go through kernel
            kernel.iter().enumerate().for_each(|(j, kernel_val)| {
                // note: coord 0 is on the ancor!
                let kernel_x = (j as u32 % kernel_info.1 as u32) as i32 - kernel_ancor_x;
                let kernel_y = if j > 0 {
                    (j as i32 / kernel_info.0 as i32) - kernel_ancor_y
                } else {
                    0 - kernel_ancor_y
                };

                let kx_in_image = img_x as i32 + kernel_x;
                let ky_in_image = img_y as i32 + kernel_y;

                // check if kernel part not out of bounds
                if !((kx_in_image < 0 || kx_in_image >= info.width as i32)
                    || (ky_in_image < 0 || ky_in_image >= info.height as i32))
                {
                    let current_pos_in_image =
                        kx_in_image * 3 + ky_in_image * 3 * info.width as i32;

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
