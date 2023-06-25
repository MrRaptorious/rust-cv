use png::{Decoder, OutputInfo};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>>{
    const IMG_PATH: &str = "res/lenna.png";
    const OUTPUT_PATH: &str = "out/lenna.png";

    let (buf, info) = load_image(IMG_PATH)?; 
    let bytes = &buf[..info.buffer_size()];

    save_image(OUTPUT_PATH, bytes, info)?;

    return Ok(());
}

/// Load the png from the specified path
fn load_image(path: &str) -> Result<(Vec<u8>, OutputInfo), Box<dyn Error>> {
    let decoder = Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    
    return Ok((buf,info))
}

/// save the image to the specified path
fn save_image(path: &str, data: &[u8], info: OutputInfo) -> Result<(),Box<dyn Error>> {
    let path = Path::new(path);
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header()?;

    writer.write_image_data(&data)?;

    return Ok(());
}
