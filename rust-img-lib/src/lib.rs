mod utils;
mod algorithms;
mod textureatlas_format;

use base64::Engine;
use image::{imageops, GenericImageView};
use utils::{set_panic_hook, encode_image_as_png};
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

#[inline]
fn row_col_from_idx(idx: u32, icons_per_row: u32) -> (u32, u32)
{
    (idx / icons_per_row, idx % icons_per_row)
}

#[wasm_bindgen]
pub fn make_icongrid_legacy(icongrid: Vec<u8>, icons: js_sys::Array) -> Vec<u8>
{
    let icon_size_default:u32 = 150;
    let mut imgvec = vec![];
    
    let arr_len = icons.length();
    for i in 0..arr_len
    {
        let imgdata = icons.get(i).as_string().unwrap();
        let icon_img = string_to_img(imgdata).unwrap();

        imgvec.push(icon_img);
    }

    let mut icongrid_img = image::load_from_memory(&icongrid).unwrap();
    
    let icons_per_row = icongrid_img.width() / icon_size_default;
    let mut rows_available = icongrid_img.height() / icon_size_default;
    let (_, _, _, down) = utils::get_bounding_box(&icongrid_img, None).unwrap();

    let bottom_row_idx = down / icon_size_default;

    let bottom_row_img = icongrid_img.crop_imm(0, icon_size_default * bottom_row_idx, icongrid_img.width(), icon_size_default);
    let (_, _, right, _) = utils::get_bounding_box(&bottom_row_img, None).unwrap();
    let right_col_idx = right / icon_size_default;

    let mut cur_total_idx = bottom_row_idx * icons_per_row + right_col_idx;

    for icon in imgvec
    {
        cur_total_idx += 1;
        let (row, col) = row_col_from_idx(cur_total_idx, icons_per_row);
        if row >= rows_available
        {
            // make new row
            let mut new_icongrid = image::DynamicImage::new_rgba8(icongrid_img.width(), (row + 1) * icon_size_default);
            imageops::overlay(&mut new_icongrid, &icongrid_img, 0, 0);
            icongrid_img = new_icongrid;
            rows_available = icongrid_img.height() / icon_size_default;
        }

        // insert as normal
        let (icon_width, icon_height) = icon.dimensions();
        let mut insertion_x = col * icon_size_default;
        let mut insertion_y = row * icon_size_default;
        if icon_width < icon_size_default
        {
            // center icon within area
            insertion_x += (icon_size_default - icon_width)/2;
        }

        if icon_height < icon_size_default
        {
            // center icon within area
            insertion_y += (icon_size_default - icon_height)/2;
        }

        imageops::overlay(&mut icongrid_img, &icon, insertion_x as i64, insertion_y as i64);
    }

    encode_image_as_png(icongrid_img)
}

// #[wasm_bindgen]
// pub fn greet(name: &str) -> String {
//     // alert("Hello, rust-img-lib!");
//     format!("Hi there, {name}")
// }
