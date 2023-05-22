use std::{collections::{HashMap, HashSet}, sync::atomic::AtomicU32};

use wasm_bindgen::prelude::*;

use crate::{utils::{PackError, encode_image_as_png, self}, algorithms::{PackingRectangle, Packer, FitRect}, textureatlas_format};
use image::{imageops, DynamicImage};
use super::helpers;

// use super::{PackingRectangle, Packer, FitRect, growingpack_fns};

// static ID_GEN: AtomicU32 = AtomicU32::new(0);

struct ImgRectInfo
{
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

struct TransformInfo
{
    scale_x: f64,
    scale_y: f64,
    flip_x: bool,
    flip_y: bool
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
    spr_id: String,
    // img_data: Vec<u8>,
    img_cache_id: u64,
    xml_data: Option<String>,
    animation_prefix: String,
    rect: Option<ImgRectInfo>,
    transform: TransformInfo,
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
    frames: Vec<FrameInfo>
}

#[wasm_bindgen]
impl GrowingPacker
{
    pub fn new() -> Self
    {
        Self {
            frame_image_cache: ImageCache::new(),
            frames: vec![]
        }
    }

    pub fn add_single_frame(
        &mut self,
        spr_id: String,
        img_data: Vec<u8>,
        animation_prefix: String,
        scale_x: f64,
        scale_y: f64,
        flip_x: bool,
        flip_y: bool,
        frame_x: i64,
        frame_y: i64,
        frame_width: u64,
        frame_height: u64,
    )
    {
        let (imghash, (left, top, _right, _bottom)) = self.frame_image_cache.add_image(
            image::load_from_memory(&img_data).expect("Should be valid image")
        );

        let cur_frameinfo = FrameInfo {
            spr_id,
            img_cache_id: imghash,
            xml_data: None,
            animation_prefix,
            rect: None,
            transform: TransformInfo { scale_x, scale_y, flip_x, flip_y },
            frame_rect: FrameRectInfo { frame_x: frame_x - (left as i64), frame_y: frame_y - (top as i64), frame_width, frame_height }
        };
        self.frames.push(cur_frameinfo);
    }

    pub fn add_spritesheet_frame(
        &mut self,
        spr_id: String,
        img_data: Vec<u8>,
        xml_data: String,
        animation_prefix: String,
        rect_x: u32,
        rect_y: u32,
        rect_width: u32,
        rect_height: u32,
        scale_x: f64,
        scale_y: f64,
        flip_x: bool,
        flip_y: bool,
        frame_x: i64,
        frame_y: i64,
        frame_width: u64,
        frame_height: u64,
    )
    {
        let (imghash, (left, top, _right, _bottom)) = self.frame_image_cache.add_image(
            image::load_from_memory(&img_data).unwrap().crop_imm(rect_x, rect_y, rect_width, rect_height)
        );

        let cur_frameinfo = FrameInfo {
            spr_id,
            img_cache_id: imghash,
            xml_data: Some(xml_data),
            animation_prefix,
            rect: Some(ImgRectInfo { x: rect_x, y: rect_y, width: rect_width, height: rect_height }),
            transform: TransformInfo { scale_x, scale_y, flip_x, flip_y },
            frame_rect: FrameRectInfo { frame_x: frame_x - (left as i64), frame_y: frame_y - (top as i64), frame_width, frame_height }
        };
        self.frames.push(cur_frameinfo);
    }

    /// Returns the final image as a PNG
    pub fn make_packed_image(&mut self) -> Vec<u8>
    {
        let (final_width, final_height, fits) = self.pack().expect("Packing error happened!!");
        let mut base = image::DynamicImage::new_rgba8(final_width, final_height);
        for fit in fits
        {
            imageops::overlay(&mut base, &self.frame_image_cache.cache[&fit.id], fit.x as i64, fit.y as i64);
        }
        encode_image_as_png(base)
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
                result.width() as u32, 
                result.height() as u32, 
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