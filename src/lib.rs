extern crate nalgebra as na;

use image::{ImageBuffer, RgbImage};
use na::SMatrix;
use std::fs::File;
use std::io::Read;

const SIZE: usize = 256;

pub type Map256 = SMatrix<u32, SIZE, SIZE>;

pub fn load_file(file_path: &std::path::PathBuf) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Can't open that file");
    let mut slice = Vec::new();
    file.read_to_end(&mut slice).unwrap();
    slice
}

pub fn generate_visualization(slice: &Vec<u8>, map: &mut Map256) {
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
