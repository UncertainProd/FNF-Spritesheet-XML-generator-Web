mod utils;
mod algorithms;

use base64::Engine;
use image::{imageops, ImageEncoder};
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook()
{
    set_panic_hook();
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn string_to_img(imgstring: String) -> Result<image::DynamicImage, image::ImageError>
{
    // decode
    let imgbytes = base64::engine::general_purpose::STANDARD.decode(imgstring).unwrap();
    image::load_from_memory(&imgbytes)
}

fn img_to_string(img: image::DynamicImage) -> String
{
    // png encode
    let mut pngbuf:Vec<u8> = Vec::new();
    let pngencoder = image::codecs::png::PngEncoder::new(&mut pngbuf);
    pngencoder.write_image(img.as_bytes(), img.width(), img.height(), img.color()).unwrap();

    // base64 encode
    base64::engine::general_purpose::STANDARD.encode(pngbuf)
}

#[wasm_bindgen]
pub fn make_icongrid(img_datas: js_sys::Array) -> String
{
    let mut imgvec = vec![];

    let mut total_width:u32 = 0;
    let mut total_height:u32 = 0;
    
    let arr_len = img_datas.length();
    alert(&format!("Array length = {}", arr_len));
    for i in 0..arr_len
    {
        let imgdata = img_datas.get(i).as_string().unwrap();
        let img = string_to_img(imgdata).unwrap();

        total_width += img.width();
        total_height = std::cmp::max(total_height, img.height());

        imgvec.push(img);
    }

    let mut base = image::DynamicImage::new_rgba8(total_width, total_height);
    let mut cur_x:u32 = 0;
    for img in imgvec
    {
        imageops::overlay(&mut base, &img, cur_x.into(), 0);
        cur_x += img.width();
    }

    img_to_string(base)
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    // alert("Hello, rust-img-lib!");
    format!("Hi there, {name}")
}
