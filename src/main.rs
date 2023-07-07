use png::{ColorType};
use std::{error::Error};

use crate::image::{binarize, create_image_from_channels};

pub mod filter;
pub mod image;
pub mod kernels;

fn main() -> Result<(), Box<dyn Error>> {
    const IMG_PATH: &str = "res/lenna.png";
    const OUTPUT_PATH: &str = "out/lenna.png";

    let (buf, info) = image::load_image(IMG_PATH)?;

    let image_type = match info.color_type {
        ColorType::Grayscale => image::ColorType::Gray,
        ColorType::Rgb => image::ColorType::Color,
        _ => {
            panic!()
        }
    };

    let image = image::Image {
        width: info.width as usize,
        height: info.height as usize,
        data: buf,
        color_type: image_type,
    };

    let kernel = kernels::get_sharpening_kernel(2.0f32);

    let image = filter::apply_kernel(&image, &kernel)?;

    let mut red_channel = image::get_channel(&image, image::Channel::R, false)?;
    let mut green_channel = image::get_channel(&image, image::Channel::G, false)?;
    let mut blue_channel = image::get_channel(&image, image::Channel::B, false)?;

    binarize(&mut red_channel, 100)?;
    binarize(&mut blue_channel, 255)?;
    binarize(&mut green_channel, 255)?;

    image::save_image("out/r.png", &red_channel)?;
    image::save_image("out/g.png", &green_channel)?;
    image::save_image("out/b.png", &red_channel)?;

    let reconstructed = create_image_from_channels(&red_channel, &green_channel, &blue_channel)?;

    image::save_image(OUTPUT_PATH, &reconstructed)?;

    Ok(())
}
