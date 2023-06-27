use png::{Decoder, OutputInfo};
use rayon::prelude::*;
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    const IMG_PATH: &str = "res/lenna.png";
    const OUTPUT_PATH: &str = "out/lenna.png";

    let (mut buf, info) = load_image(IMG_PATH)?;

    to_gray(&mut buf);

    save_image(OUTPUT_PATH, &buf, info)?;

    Ok(())
}

/// Load the png from the specified path
fn load_image(path: &str) -> Result<(Vec<u8>, OutputInfo), Box<dyn Error>> {
    let decoder = Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

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
