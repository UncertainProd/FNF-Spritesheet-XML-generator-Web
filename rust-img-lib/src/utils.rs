use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use image::{ImageEncoder, GenericImageView, imageops};

use crate::algorithms::spritesheetpackers::growingpacker::TransformInfo;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn encode_image_as_png(img: image::DynamicImage) -> Vec<u8>
{
    let mut out_vec = Vec::new();
    let png_encoder = image::codecs::png::PngEncoder::new(&mut out_vec);
    png_encoder.write_image(img.as_bytes(), img.width(), img.height(), img.color()).expect("Error writing png to buffer!");
    out_vec
}

pub fn get_hash_from_image_bytes(img_bytes: &[u8]) -> u64
{
    let mut hasher = DefaultHasher::new();
    img_bytes.hash(&mut hasher);
    hasher.finish()
}

/// A Rust port of the `getbbox()` function from Python's PIL library
/// 
/// Returns a tuple containing (left, top, right, bottom) coords of the image. If the entire image is transparent (aka less than the `alpha_threshold`) then it returns `None`
/// 
/// Refer [`GETBBOX`](https://github.com/python-pillow/Pillow/blob/7e8b11b1593583069bd12ba1d541a42940d669f6/src/libImaging/GetBBox.c#L33) for the main reference behind this algorithm
pub fn get_bounding_box(img: &image::DynamicImage, alpha_threshold: Option<u8>) -> Option<(u32, u32, u32, u32)>
{
    let alpha_threshold = alpha_threshold.unwrap_or(0);
    let mut bb_left: u32 = img.width();
    let mut bb_right: u32 = 0;
    let mut bb_top: i64 = -1;
    let mut bb_bottom: u32 = 0;
    for (x, y, color) in img.pixels()
    {
        if color.0[3] > alpha_threshold
        {
            if x < bb_left
            {
                bb_left = x;
            }

            if x >= bb_right {
                bb_right = x + 1;
            }

            if bb_top < 0
            {
                bb_top = y as i64;
            }
            bb_bottom = y + 1;
        }
    }

    if bb_top < 0
    {
        return None;
    }

    Some((bb_left, bb_top as u32, bb_right, bb_bottom))
}

pub fn transform_image(img: image::DynamicImage, img_transform: TransformInfo) -> image::DynamicImage
{
    // first we scale then we flip
    let mut new_img = img;
    if img_transform.new_width != new_img.width() || img_transform.new_height != new_img.height()
    {
        new_img = new_img.resize_exact(
            img_transform.new_width, 
            img_transform.new_height, 
            imageops::FilterType::Nearest
        );
    }

    if img_transform.flip_x
    {
        new_img = new_img.fliph();
    }
    if img_transform.flip_y
    {
        new_img = new_img.flipv();
    }

    return new_img;
}

pub fn pad_image_uniform(img: image::DynamicImage, padding: u32) -> image::DynamicImage
{
    let mut padded_img = image::DynamicImage::new_rgba8(img.width() + 2*padding, img.height() + 2*padding);
    imageops::overlay(&mut padded_img, &img, padding as i64, padding as i64);
    padded_img
}

#[derive(Debug)]
pub struct PackError;

impl std::fmt::Display for PackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred while packing. Most likely due to invalid sorting")
    }
}
impl std::error::Error for PackError {}