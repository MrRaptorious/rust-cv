use png::{Decoder, OutputInfo};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn main() {
    const IMG_PATH: &str = "res/lenna.png";
    const OUTPUT_PATH: &str = "out/lenna.png";

    let decoder = Decoder::new(File::open(IMG_PATH).unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];

    save_image(OUTPUT_PATH, bytes, info)
}

fn save_image(path: &str, data: &[u8], info: OutputInfo) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, info.width, info.height);
    encoder.set_color(info.color_type);
    encoder.set_depth(info.bit_depth);

    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap();
}
