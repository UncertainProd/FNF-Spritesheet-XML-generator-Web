use crate::{algorithms::FitRect, utils::encode_image_as_png};

use super::Packer;
use image::{self, imageops};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct IconPacker
{
    images: Vec<image::DynamicImage>
}

#[wasm_bindgen]
impl IconPacker
{
    pub fn new() -> Self
    {
        Self { images: vec![] }
    }

    pub fn add_image(&mut self, image_buffer: JsValue)
    {
        let img_array = js_sys::Uint8Array::new(&image_buffer);
        let img_bytes = img_array.to_vec();
        self.images.push(image::load_from_memory(&img_bytes).expect("Error loading image from memory!"));
    }

    /// Returns the final image as a PNG
    pub fn make_packed_image(&mut self) -> Vec<u8>
    {
        let (final_width, final_height, fits) = self.pack().expect("Big Packing error!");
        let mut base = image::DynamicImage::new_rgba8(final_width, final_height);
        for fit in fits
        {
            // Note: fit.id is the index in the array so the following code does make sense (but only here) :)
            imageops::overlay(&mut base, &self.images[fit.id as usize], fit.x as i64, fit.y as i64);
        }
        encode_image_as_png(&base)
    }
}

impl Packer for IconPacker
{
    fn pack(&mut self) -> Result<(u32, u32, Vec<super::FitRect>), crate::utils::PackError>
    {
        let mut fits = vec![];
        let mut cur_x: u32 = 0; // this is also going to be the final width
        let mut final_height: u32 = 0;
        for (i, rect) in self.images.iter().enumerate()
        {
            fits.push(FitRect::new(cur_x, 0, rect.width(), rect.height(), i as u64));
            cur_x += rect.width();
            final_height = std::cmp::max(final_height, rect.height());
        }
        Ok((cur_x, final_height, fits))
    }
}