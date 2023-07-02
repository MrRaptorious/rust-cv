use png::{BitDepth, ColorType, Decoder, OutputInfo};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

mod filter;

fn main() -> Result<(), Box<dyn Error>> {
    const IMG_PATH: &str = "res/lenna.png";
    const OUTPUT_PATH: &str = "out/lenna.png";

    // const IMG_PATH: &str = "res/test_small.png";
    // const OUTPUT_PATH: &str = "out/test_small.png";

    let (buf, info) = load_image(IMG_PATH)?;

    let gaussian_kernel =  filter::get_gaussian_kernel();

    let filterd_image = filter::apply_kernel(&buf, (info.width,info.height), &gaussian_kernel)?;

    save_image(OUTPUT_PATH, &filterd_image, info)?;

    Ok(())
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
