use core::fmt;
use png::{BitDepth, Decoder, OutputInfo};
use rayon::prelude::*;
use std::{error::Error, fs::File, io::BufWriter, path::Path};

#[derive(Copy, Clone, PartialEq)]
pub enum ColorType {
    Gray,
    Color,
}

pub enum Channel {
    Gray,
    R,
    G,
    B,
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
    pub fn get_pixl_width(&self) -> usize {
        match self.color_type {
            ColorType::Gray => 1,
            ColorType::Color => 3,
        }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

// Creates an black and white version of the image
pub fn to_gray(image: &mut Image) -> Result<(), Box<dyn Error>> {
    match image.color_type {
        ColorType::Gray => Err(Box::new(ImageError::new("The image is already gray."))),
        ColorType::Color => {
            let mut gray_img = vec![0; image.data.len() / 3];

            gray_img.par_iter_mut().enumerate().for_each(|(i, pxl)| {
                let original_pxl = &image.data[i * 3..i * 3 + 3];
                let gray_color: u8 =
                    (original_pxl.iter().map(|x| *x as u16).sum::<u16>() / 3) as u8;

                *pxl = gray_color;
            });

            image.data = gray_img;
            image.color_type = ColorType::Gray;

            Ok(())
        }
    }
}

// Binarize the image, if the image is not grayscale it will be converted before
pub fn binarize(image: &mut Image, threshold: u8) -> Result<(), Box<dyn Error>> {
    if matches!(image.color_type, ColorType::Color) {
        to_gray(image)?;
    }

    image
        .data
        .iter_mut()
        .for_each(|x| *x = if *x > threshold { 255 } else { 0 });

    Ok(())
}

/// Strips one channel out of the Image. If the image is of type Gray, it just will be cloned.
/// # Arguments
/// * `image` - The image to get the channel from
/// * `channel` - The channel to receive
/// * `in_color` - if true the returned image will be RGB but only contain one channel
/// else only a gray image will be returned
pub fn get_channel(
    image: &Image,
    channel: Channel,
    in_color: bool,
) -> Result<Image, Box<dyn Error>> {
    match image.color_type {
        ColorType::Color => {
            let pixel_index = match channel {
                Channel::R => 0,
                Channel::G => 1,
                Channel::B => 2,
                _ => {
                    return Err(Box::new(ImageError::new(
                        "Color format of Image not supported!",
                    )))
                }
            };

            if in_color {
                let mut channel_image: Vec<u8> = vec![0; image.data.len()];
                channel_image
                    .chunks_mut(image.get_pixl_width())
                    .zip(image.data.chunks(image.get_pixl_width()))
                    .for_each(|(channel_pix, color_pix)| {
                        channel_pix[pixel_index] = color_pix[pixel_index];
                    });

                Ok(Image {
                    width: image.width,
                    height: image.height,
                    color_type: ColorType::Color,
                    data: channel_image,
                })
            } else {
                let mut channel_image: Vec<u8> = vec![0; image.data.len() / 3];
                channel_image
                    .iter_mut()
                    .zip(image.data.chunks(image.get_pixl_width()))
                    .for_each(|(channel_pix, color_pix)| {
                        *channel_pix = color_pix[pixel_index];
                    });

                Ok(Image {
                    width: image.width,
                    height: image.height,
                    color_type: ColorType::Gray,
                    data: channel_image,
                })
            }
        }
        _ => Err(Box::new(ImageError::new(
            "Color format of Image not supported!",
        ))),
    }
}

pub fn create_image_from_channels(
    red: &Image,
    green: &Image,
    blue: &Image,
) -> Result<Image, Box<dyn Error>> {
    if ![red.color_type, green.color_type, blue.color_type]
        .iter()
        .all(|x| *x == ColorType::Gray)
        || red.shape() != green.shape()
        || green.shape() != blue.shape()
    {
        return Err(Box::new(ImageError::new("Not all channels are gray and/or have the same shape!")));
    }

    let mut image_data: Vec<u8> = vec![0; red.data.len() * 3];

    image_data.chunks_exact_mut(3).enumerate().for_each(|(i, pix)| {
        pix[0] = red.data[i];
        pix[1] = green.data[i];
        pix[2] = blue.data[i];
    });

    Ok(Image {
        data: image_data,
        width: red.width,
        height: red.height,
        color_type: ColorType::Color,
    })
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
