use std::{str::FromStr, fmt::Debug, path::Path, io::Write};

#[allow(unused_imports)]
use quick_xml::{Reader, events::{Event, attributes::Attribute, BytesStart, BytesDecl, BytesText, BytesEnd}, Writer};

use crate::utils::PrefixCounter;

#[allow(dead_code)]
fn parse_value<T: FromStr>(att: &Attribute) -> T where <T as FromStr>::Err: Debug
{
    String::from_utf8(att.value.as_ref().to_vec()).unwrap().parse().unwrap()
}

fn zero_fill_num(num: u32, to_len: usize) -> Result<String, String>
{
    let num_string = num.to_string();
    let n_zeros_to_insert: isize = to_len as isize - num_string.len() as isize;
    if n_zeros_to_insert < 0
    {
        return Err(num_string);
    }
    else
    {
        return Ok(format!("{}{}", "0".repeat(n_zeros_to_insert as usize), num_string));
    }
}

// #[derive(Debug)]
pub struct TextureAtlas
{
    pub image_path: String,
    pub subtextures: Vec<SubTexture>,
}

impl Default for TextureAtlas
{
    fn default() -> Self {
        Self { image_path: "".to_string(), subtextures: vec![] }
    }
}

impl TextureAtlas
{
    #[cfg(not(target_arch="wasm32"))]
    pub fn from_xml_path<P: AsRef<Path>>(path: P) -> Self
    {
        let mut reader = Reader::from_file(path).expect("Unable to open the xml file!");
        reader.trim_text(true);

        let mut buf = Vec::new();

        let mut img_path: String = String::from("");
        let mut subtextures = vec![];
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Eof) => break,
                
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"TextureAtlas" => {
                            let imgname = e.attributes().find(|item| item.as_ref().unwrap().key.0 == b"imagePath");
                            img_path = String::from_utf8(imgname.unwrap().unwrap().value.as_ref().to_vec()).unwrap();
                        },
                        _ => ()
                    }
                },

                Ok(Event::Empty(e)) => {
                    if e.name().as_ref() == b"SubTexture"
                    {
                        let mut st = SubTexture::default();
                        for att in e.attributes()
                        {
                            let att = att.unwrap();
                            match att.key.as_ref() {
                                b"name" => st.name = String::from_utf8(att.value.as_ref().to_vec()).unwrap(),
                                b"x" => st.x = parse_value(&att),
                                b"y" => st.y = parse_value(&att),
                                b"width" => st.width = parse_value(&att),
                                b"height" => st.height = parse_value(&att),
                                b"frameX" => st.frame_x = Some(parse_value(&att)),
                                b"frameY" => st.frame_y = Some(parse_value(&att)),
                                b"frameWidth" => st.frame_width = Some(parse_value(&att)),
                                b"frameHeight" => st.frame_height = Some(parse_value(&att)),
                                _ => ()
                            }
                        }
                        subtextures.push(st);
                    }
                },

                _ => ()
            }

            buf.clear();
        }

        Self { image_path: img_path, subtextures }
    }

    #[cfg(not(target_arch="wasm32"))]
    pub fn from_xml_string(xmlstr: &str) -> Self
    {
        let mut reader = Reader::from_str(xmlstr);
        reader.trim_text(true);

        let mut img_path: String = String::from("");
        let mut subtextures = vec![];
        loop {
            match reader.read_event() {
                Ok(Event::Eof) => break,
                
                Ok(Event::Start(e)) => {
                    match e.name().as_ref() {
                        b"TextureAtlas" => {
                            let imgname = e.attributes().find(|item| item.as_ref().unwrap().key.0 == b"imagePath");
                            img_path = String::from_utf8(imgname.unwrap().unwrap().value.as_ref().to_vec()).unwrap();
                        },
                        _ => ()
                    }
                },

                Ok(Event::Empty(e)) => {
                    if e.name().as_ref() == b"SubTexture"
                    {
                        let mut st = SubTexture::default();
                        for att in e.attributes()
                        {
                            let att = att.unwrap();
                            match att.key.as_ref() {
                                b"name" => st.name = String::from_utf8(att.value.as_ref().to_vec()).unwrap(),
                                b"x" => st.x = parse_value(&att),
                                b"y" => st.y = parse_value(&att),
                                b"width" => st.width = parse_value(&att),
                                b"height" => st.height = parse_value(&att),
                                b"frameX" => st.frame_x = Some(parse_value(&att)),
                                b"frameY" => st.frame_y = Some(parse_value(&att)),
                                b"frameWidth" => st.frame_width = Some(parse_value(&att)),
                                b"frameHeight" => st.frame_height = Some(parse_value(&att)),
                                _ => ()
                            }
                        }
                        subtextures.push(st);
                    }
                },

                _ => ()
            }
        }

        Self { image_path: img_path, subtextures }
    }

    pub fn write_to<W: Write>(&self, writer: W)
    {
        let mut prefix_counter: PrefixCounter = PrefixCounter::new();
        let mut wr = Writer::new_with_indent(writer, '\t' as u8, 1);
        
        // xml decl
        wr.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None))).unwrap();

        // <TextureAtlas>...
        let mut start_tag = BytesStart::new("TextureAtlas");
        start_tag.push_attribute(("imagePath", self.image_path.as_str()));
        wr.write_event(Event::Start(start_tag)).unwrap();

        // le shameless plug :)
        wr.write_event(Event::Comment(BytesText::new(" Created using the Spritesheet and XML generator "))).unwrap();
        wr.write_event(Event::Comment(BytesText::new(" https://uncertainprod.github.io/FNF-Spritesheet-XML-generator-Web "))).unwrap();

        for thing in &self.subtextures
        {
            let anim_suffix_num = prefix_counter.add_prefix(&thing.name);

            let mut bys = BytesStart::new("SubTexture");
            let suffix_num = zero_fill_num(anim_suffix_num, 4).unwrap_or(anim_suffix_num.to_string());
            bys.push_attribute(("name", format!("{}{}", thing.name, suffix_num).as_str()));
            bys.push_attribute(("x", thing.x.to_string().as_str()));
            bys.push_attribute(("y", thing.y.to_string().as_str()));
            bys.push_attribute(("width", thing.width.to_string().as_str()));
            bys.push_attribute(("height", thing.height.to_string().as_str()));

            if let Some(fx) = thing.frame_x {
                bys.push_attribute(("frameX", fx.to_string().as_str()));
            }
            if let Some(fy) = thing.frame_y {
                bys.push_attribute(("frameY", fy.to_string().as_str()));
            }
            if let Some(fw) = thing.frame_width {
                bys.push_attribute(("frameWidth", fw.to_string().as_str()));
            }
            if let Some(fh) = thing.frame_height {
                bys.push_attribute(("frameHeight", fh.to_string().as_str()));
            }

            if let Some(flipx) = thing.flip_x {
                bys.push_attribute(("flipX", flipx.to_string().as_str()));
            }
            if let Some(flipy) = thing.flip_y {
                bys.push_attribute(("flipY", flipy.to_string().as_str()));
            }

            wr.write_event(Event::Empty(bys)).unwrap();
        }

        wr.write_event(Event::End(BytesEnd::new("TextureAtlas"))).unwrap();
    }
}

// #[derive(Debug, PartialEq, Eq, PartialOrd)]
#[derive(PartialEq, Eq, PartialOrd, Clone)]
pub struct SubTexture
{
    pub name: String,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub frame_x: Option<i32>,
    pub frame_y: Option<i32>,
    pub frame_width: Option<u32>,
    pub frame_height: Option<u32>,
    pub flip_x: Option<bool>,
    pub flip_y: Option<bool>
}

impl Ord for SubTexture {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl SubTexture
{
    pub fn new(name: String, x: u32, y: u32, width: u32, height: u32, frame_x: Option<i32>, frame_y: Option<i32>, frame_width: Option<u32>, frame_height: Option<u32>, flip_x: Option<bool>, flip_y: Option<bool>) -> Self
    {
        Self { name, x, y, width, height, frame_x, frame_y, frame_width, frame_height, flip_x, flip_y }
    }
}

impl Default for SubTexture {
    fn default() -> Self {
        Self { name: "".to_string(), x: 0, y: 0, width: 0, height: 0, frame_x: None, frame_y: None, frame_width: None, frame_height: None, flip_x: None, flip_y: None }
    }
}