extern crate nalgebra as na;

use image::{ImageBuffer, RgbImage};
use na::SMatrix;
use std::fs::File;
use std::io::Read;

const SIZE: usize = 256;
type Matrix256x256u = SMatrix<u32, SIZE, SIZE>;

fn load_file(file_path: &str) -> Vec<u8> {
    let mut file = File::open(file_path).expect("Can't open that file");
    let mut slice = Vec::new();
    file.read_to_end(&mut slice).unwrap();
    slice
}

fn generate_visualization(slice: &Vec<u8>, map: &mut Matrix256x256u) {
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

fn map_to_image(map: &Matrix256x256u) -> RgbImage {
    let mut img: RgbImage = ImageBuffer::new(SIZE as u32, SIZE as u32);
    for i in 0..SIZE {
        for j in 0..SIZE {
            img.put_pixel(i as u32, j as u32, image::Rgb([map[(i, j)] as u8, 0, 0]));
        }
    }
    img
}

fn main() {
    // Open target file
    let file_path = "./target/debug/bvtool";
    let slice = load_file(&file_path);

    // Create the 256x256 map
    let mut map = Matrix256x256u::zeros();

    // Generate the visualization
    generate_visualization(&slice, &mut map);

    // Create the image
    let img: RgbImage = map_to_image(&map);

    // Save the image
    img.save("data/heatmap.png").unwrap();
}

fn _gen_exec_viz() {
    let paths = std::env::var("PATH").unwrap();
    let paths: Vec<&str> = paths.split(':').collect();
    for path in paths {
        let dir = std::fs::read_dir(path).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let file_path = entry.path();
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            println!("[LOADING] {}", file_name);
            let slice = load_file(&file_path.to_str().unwrap());
            let mut map = Matrix256x256u::zeros();

            println!("[GENERATING] generating 256x256 map");
            generate_visualization(&slice, &mut map);
            let img: RgbImage = map_to_image(&map);

            let out_path = format!("data/exec/{}.bvtool.png", file_name);
            println!("[DONE] saving map into {}", &out_path);
            img.save(out_path).unwrap();
        }
    }
}
