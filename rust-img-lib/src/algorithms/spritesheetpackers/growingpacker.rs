use std::{collections::HashMap, io::{self, Write}};

use wasm_bindgen::prelude::*;

use crate::{utils::{PackError, encode_image_as_png, self, transform_image}, algorithms::{PackingRectangle, Packer, FitRect}, textureatlas_format::{self, SubTexture}};
use image::{imageops, DynamicImage};
use super::helpers;

// use super::{PackingRectangle, Packer, FitRect, growingpack_fns};

pub struct TransformInfo
{
    pub new_width: u32,
    pub new_height: u32,
    pub flip_x: bool,
    pub flip_y: bool
}

struct FrameRectInfo
{
    frame_x: i64,
    frame_y: i64,
    frame_width: u64,
    frame_height: u64
}

struct FrameInfo
{
    // spr_id: String,
    // img_data: Vec<u8>,
    // img_cache_id: u64,
    animation_prefix: String,
    frame_rect: FrameRectInfo
}

struct ImageCache
{
    cache: HashMap<u64, DynamicImage>
}

impl ImageCache {
    pub fn new() -> Self
    {
        Self { cache: HashMap::new() }
    }

    pub fn add_image(&mut self, img: DynamicImage) -> (u64, (u32, u32, u32, u32))
    {
        let bounds_opt = utils::get_bounding_box(&img, None);
        match bounds_opt {
            Some((left, top, right, bottom)) => {
                let cropped_img = img.crop_imm(left, top, right - left, bottom - top);
                let imghash = utils::get_hash_from_image_bytes(cropped_img.as_bytes());
                if !self.cache.contains_key(&imghash)
                {
                    self.cache.insert(imghash, cropped_img);
                }
                return (imghash, (left, top, right, bottom));
            }
            None => {
                todo!()
            }
        }
    }
}

#[wasm_bindgen]
pub struct GrowingPacker
{
    frame_image_cache: ImageCache,
    frames: HashMap<u64, Vec<FrameInfo>>,
    _spritesheet_store: HashMap<String, image::DynamicImage>
}

#[wasm_bindgen]
impl GrowingPacker
{
    pub fn new() -> Self
    {
        Self {
            frame_image_cache: ImageCache::new(),
            frames: HashMap::new(),
            _spritesheet_store: HashMap::new()
        }
    }

    pub fn add_image_to_store(&mut self, img_key: String, img_data: Vec<u8>)
    {
        self._spritesheet_store.insert(img_key, image::load_from_memory(&img_data).expect("Expected valid image. Got invalid image!"));
    }

    pub fn add_single_frame(
        &mut self,
        // spr_id: String,
        img_data: Vec<u8>,
        animation_prefix: String,
        new_width: u32,
        new_height: u32,
        flip_x: bool,
        flip_y: bool,
        frame_x: i64,
        frame_y: i64,
        frame_width: u64,
        frame_height: u64,
    )
    {
        let true_img = transform_image(image::load_from_memory(&img_data).expect("Should be valid image"), TransformInfo { new_width, new_height, flip_x, flip_y });
        let (imghash, (left, top, _right, _bottom)) = self.frame_image_cache.add_image(
            true_img
        );

        let cur_frameinfo = FrameInfo {
            // spr_id,
            // img_cache_id: imghash,
            animation_prefix,
            frame_rect: FrameRectInfo { frame_x: frame_x - (left as i64), frame_y: frame_y - (top as i64), frame_width, frame_height }
        };
        
        let imgframes = self.frames.get_mut(&imghash);
        match imgframes {
            Some(frames) => {
                frames.push(cur_frameinfo);
            }
            None => {
                self.frames.insert(imghash, vec![ cur_frameinfo ]);
            }
        }
    }

    pub fn add_spritesheet_frame(
        &mut self,
        // spr_id: String,
        spritesheet_id: String,
        animation_prefix: String,
        rect_x: u32,
        rect_y: u32,
        rect_width: u32,
        rect_height: u32,
        new_width: u32,
        new_height: u32,
        flip_x: bool,
        flip_y: bool,
        frame_x: i64,
        frame_y: i64,
        frame_width: u64,
        frame_height: u64,
    )
    {
        let pre_img = self._spritesheet_store
            .get(&spritesheet_id)
            .expect("Key not in map!")
            .crop_imm(rect_x, rect_y, rect_width, rect_height);

        
        let true_img = transform_image(pre_img, TransformInfo { new_width, new_height, flip_x, flip_y });
        let (imghash, (left, top, _right, _bottom)) = self.frame_image_cache.add_image(
            true_img
        );

        let cur_frameinfo = FrameInfo {
            // spr_id,
            // img_cache_id: imghash,
            animation_prefix,
            frame_rect: FrameRectInfo { frame_x: frame_x - (left as i64), frame_y: frame_y - (top as i64), frame_width, frame_height }
        };

        let imgframes = self.frames.get_mut(&imghash);
        match imgframes {
            Some(frames) => {
                frames.push(cur_frameinfo);
            }
            None => {
                self.frames.insert(imghash, vec![ cur_frameinfo ]);
            }
        }
    }

    /// Returns the final image as a PNG
    pub fn make_packed_image(&mut self) -> Vec<u8>
    {
        let (final_width, final_height, fits) = self.pack().expect("Packing error happened!!");
        let mut base = image::DynamicImage::new_rgba8(final_width, final_height);

        let mut xml_bytes = Vec::new();
        let mut texture_atlas = textureatlas_format::TextureAtlas::default();
        texture_atlas.image_path = "testing.png".to_string();
        
        // group frames by id
        for fit in fits
        {
            imageops::overlay(&mut base, &self.frame_image_cache.cache[&fit.id], fit.x as i64, fit.y as i64);
            let frame_group = self.frames.get(&fit.id);
            if let Some(frames) = frame_group {
                for f in frames
                {
                    texture_atlas.subtextures.push(
                        SubTexture::new(
                            f.animation_prefix.clone(), 
                            fit.x, 
                            fit.y, 
                            fit.width, 
                            fit.height, 
                            Some(f.frame_rect.frame_x as i32), 
                            Some(f.frame_rect.frame_y as i32), 
                            Some(f.frame_rect.frame_width as u32), 
                            Some(f.frame_rect.frame_height as u32)
                        )
                    );
                }
            }
        }
        texture_atlas.write_to(&mut xml_bytes);
        
        let pngbytes = encode_image_as_png(base);
        
        let mut zip_buf: Vec<u8> = Vec::with_capacity(pngbytes.len());
        let zipcursor = io::Cursor::new(&mut zip_buf);
        
        let mut zip_writer = zip::ZipWriter::new(zipcursor);
        let zip_opts = zip::write::FileOptions::default();
        
        zip_writer.start_file("testfile.png", zip_opts).expect("Could not write to zip!");
        zip_writer.write_all(&pngbytes).expect("Zip error!");

        zip_writer.start_file("testfile.xml", zip_opts).expect("Could not write to zip!");
        zip_writer.write_all(&xml_bytes).expect("Zip error!");

        zip_writer.finish().expect("Error finising zip!");
        drop(zip_writer);

        return zip_buf;
    }
}

impl Packer for GrowingPacker
{
    fn pack(&mut self) -> Result<(u32, u32, Vec<FitRect>), PackError>
    {
        let mut rects = vec![];
        for (imghash, img) in self.frame_image_cache.cache.iter()
        {
            rects.push(PackingRectangle{
                width: img.width(),
                height: img.height(),
                id: *imghash
            });
        }
        let result = helpers::bin_pack(rects.into_iter(), |im| (im.width*im.height) as i32)?;
        
        Ok(
            (
                result.width(), 
                result.height(), 
                result.items
                .into_iter()
                .map(
                    |(i, elem)| FitRect::new(
                        elem.x as u32, elem.y as u32, 
                        elem.width as u32, elem.height as u32, 
                        i
                    )
                )
                .collect()
            )
        )
    }
}