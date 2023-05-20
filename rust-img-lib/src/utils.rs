use std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}};

use image::ImageEncoder;

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

pub fn _get_hash_from_image_bytes(img_bytes: &Vec<u8>) -> u64
{
    let mut hasher = DefaultHasher::new();
    img_bytes.hash(&mut hasher);
    hasher.finish()
}

#[derive(Debug)]
pub struct PackError;

impl std::fmt::Display for PackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "An error occurred while packing. Most likely due to invalid sorting")
    }
}
impl std::error::Error for PackError {}