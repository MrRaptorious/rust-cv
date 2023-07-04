use png::{ColorType};
use std::error::Error;


pub mod filter;
pub mod image;

fn main() -> Result<(), Box<dyn Error>> {
    const IMG_PATH: &str = "res/lenna.png";
    const OUTPUT_PATH: &str = "out/lenna.png";

    // const IMG_PATH: &str = "res/test_small.png";
    // const OUTPUT_PATH: &str = "out/test_small.png";

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

    let red_channel = image::get_channel(&image, image::Channel::R, false)?;
    let green_channel = image::get_channel(&image, image::Channel::G, false)?;
    let blue_channel = image::get_channel(&image, image::Channel::B, false)?;


    image::save_image("out/r.png", &red_channel)?;
    image::save_image("out/g.png", &green_channel)?;
    image::save_image("out/b.png", &blue_channel)?;

    image::save_image(OUTPUT_PATH, &image)?;

    Ok(())
}
