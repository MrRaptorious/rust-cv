use core::fmt;
use png::{OutputInfo, BitDepth, Decoder};
use rayon::prelude::*;
use std::{error::Error, fs::File, path::Path, io::BufWriter};

pub enum ColorType {
    Gray,
    Color,
}
#[derive(Debug)]
struct ImageError {
    message: String,
}

impl ImageError {
    fn new(message: &str) -> ImageError {
        ImageError {
            message: message.to_string(),
        }
    }
}

impl Error for ImageError {}

impl fmt::Display for ImageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub color_type: ColorType,
    pub data: Vec<u8>,
}

impl Image {
    pub fn get_pixl_width(&self) -> usize{
        match self.color_type {
            ColorType::Gray => {1}
            ColorType::Color => {3}
        }
    }
}

// Creates an black and white version of the image
pub fn to_gray(image: &Image) -> Result<Image, Box<dyn Error>> {
    match image.color_type {
        ColorType::Gray => Err(Box::new(ImageError::new("The image is already gray."))),
        ColorType::Color => {

            let mut gray_img = vec![0; image.data.len()/3];

            gray_img.par_iter_mut().enumerate().for_each(|(i,pxl)|{
                let original_pxl = &image.data[i*3..i*3+3];
                let gray_color: u8 = (original_pxl.iter().map(|x| *x as u16).sum::<u16>() / 3) as u8;

                *pxl = gray_color;
            });

            Ok(Image {
                width: image.width,
                height: image.height,
                color_type: ColorType::Gray,
                data: gray_img,
            })
        }
    }
}

/// Load the png from the specified path
pub fn load_image(path: &str) -> Result<(Vec<u8>, OutputInfo), Box<dyn Error>> {
    let decoder = Decoder::new(File::open(path)?);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;

    // only allow certain pngs
    // have to have 3 channel with 8 bit depth
    if info.color_type != png::ColorType::Rgb || info.bit_depth != BitDepth::Eight {
        return Err("PNG format not supported, only 8 Bit depth and RGB images!"
            .to_string()
            .into());
    }

    Ok((buf, info))
}

/// save the image to the specified path
pub fn save_image(path: &str, image: &Image) -> Result<(), Box<dyn Error>> {
    let path = Path::new(path);
    let file = File::create(path)?;
    let w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image.width as u32, image.height as u32);
    encoder.set_color(match image.color_type {
        ColorType::Gray => png::ColorType::Grayscale,
        ColorType::Color => png::ColorType::Rgb,
    });
    encoder.set_depth(BitDepth::Eight);

    let mut writer = encoder.write_header()?;

    writer.write_image_data(&image.data)?;

    Ok(())
}