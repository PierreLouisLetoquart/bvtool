extern crate nalgebra as na;

use image::{ImageBuffer, RgbImage};
use na::SMatrix;

#[cfg(not(target_arch = "wasm32"))]
use std::fs::File;
#[cfg(not(target_arch = "wasm32"))]
use std::io::Read;

#[cfg(target_arch = "wasm32")]
use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const SIZE: usize = 256;

pub type Map256 = SMatrix<u32, SIZE, SIZE>;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_file(file_path: &std::path::PathBuf) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Can't open that file");
    let mut slice = Vec::new();
    file.read_to_end(&mut slice).unwrap();
    slice
}

pub fn generate_visualization(slice: &[u8], map: &mut Map256) {
    // Read with a window of two the entire file
    for it in slice.windows(2) {
        map[(it[0] as usize, it[1] as usize)] += 1;
    }

    // Find the max value
    let mut max = 0.0;

    for i in 0..SIZE {
        for j in 0..SIZE {
            let f: f32 = if map[(i, j)] > 0 {
                (map[(i, j)] as f32).log10()
            } else {
                0.0
            };
            if f > max {
                max = f;
            }
        }
    }

    // Normalize the map
    for i in 0..SIZE {
        for j in 0..SIZE {
            let f: f32 = if map[(i, j)] > 0 {
                (map[(i, j)] as f32).log10()
            } else {
                0.0
            };
            map[(i, j)] = (f / max * 255.0) as u32;
        }
    }
}

pub fn map_to_image(map: &Map256) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(SIZE as u32, SIZE as u32);
    for i in 0..SIZE {
        for j in 0..SIZE {
            img.put_pixel(
                i as u32,
                j as u32,
                // image::Rgb([map[(i, j)] as u8, map[(i, j)] as u8, map[(i, j)] as u8]),
                image::Rgb([map[(i, j)] as u8, 0, 0]),
            );
        }
    }
    img
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn visualize_png(bytes: &[u8]) -> Vec<u8> {
    let mut map = Map256::zeros();
    generate_visualization(bytes, &mut map);
    let img: RgbImage = map_to_image(&map);

    let mut out = Vec::new();
    let encoder = PngEncoder::new(&mut out);
    encoder
        .encode(img.as_raw(), SIZE as u32, SIZE as u32, ColorType::Rgb8)
        .unwrap();
    out
}
