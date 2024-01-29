use std::env;
use std::fs;
use std::io::Write;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let file_path = env::args().nth(1).expect("No file path provided");

    let bytes = fs::read(&file_path).expect("Failed to read file");

    let mut map: Vec<Vec<u32>> = vec![vec![0; WIDTH]; HEIGHT];

    println!("Processing file: {}", file_path);
    println!("Bytes: {}", bytes.len());

    for window in bytes.windows(2) {
        let x = window[0] as usize;
        let y = window[1] as usize;
        map[y][x] += 1;
    }

    let mut max = 0.0;

    map.iter().for_each(|row| {
        row.iter().for_each(|v| {
            let f: f32 = if *v > 0 { (*v as f32).log10() } else { 0.0 };
            if f > max {
                max = f;
            }
        })
    });

    let mut pixels: Vec<Vec<u32>> = vec![vec![0; WIDTH]; HEIGHT];

    pixels.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, v)| {
            let t = map[y][x] as f32 / max;
            let b = (t * 255.0) as u32;
            *v = b;
        })
    });

    let mut file: fs::File = fs::File::create("outputs/out.ppm").expect("Failed to create file");

    let _ = file.write_all(&format!("P6\n256 256\n255\n").as_bytes());

    pixels.iter().for_each(|row| {
        row.iter().for_each(|v| {
            let b = *v as u8;
            let _ = file.write_all(&[b, b, b]);
        })
    });

    println!("Output written to outputs/out.ppm");
}
