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

    let gray_image = image::to_gray(&image)?;    

    // let gray_image = image::to_gray(image)?;
    let kernel =  filter::get_gaussian_kernel();

    // // view kernel
    kernel.print();

    // // filter b/w image
    let filterd_image = filter::apply_kernel(gray_image, &kernel)?;

    image::save_image(OUTPUT_PATH, &filterd_image)?;

    Ok(())
}
