extern crate image;
extern crate serde;
extern crate serde_json;

use image::{ImageBuffer, Rgba};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Serialize, Deserialize)]
struct TextInfo {
    text: String,
    position: (u32, u32),
    size: u32,
}

#[derive(Serialize, Deserialize)]
struct Ticf {
    image_data: Vec<u8>,
    text_info: TextInfo,
}

impl Ticf {
    fn new(image_data: Vec<u8>, text_info: TextInfo) -> Self {
        Self { image_data, text_info }
    }

    fn to_file(&self, path: &str) {
        let serialized = serde_json::to_string(self).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    fn from_file(path: &str) -> Self {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        serde_json::from_str(&contents).unwrap()
    }
}

fn main() {
    let ticf = Ticf::new(
        vec![0; 10000],
        TextInfo {
            text: "Hello, world".to_string(),
            position: (0, 0),
            size: 12,
        },
    );

    ticf.to_file("test.ticf");

    let ticf = Ticf::from_file("test.ticf");

    println!("{}", ticf.text_info.text);
}
