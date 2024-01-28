use std::env;
use std::fs;
use std::io::Write;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

fn main() {
    let file_path = env::args().nth(1).expect("No file path provided");
    let out_path = env::args().nth(2).expect("No output path provided");

    let bytes = fs::read(file_path).expect("Failed to read file");

    let mut map: Vec<u8> = vec![0; WIDTH * HEIGHT];

    for window in bytes.windows(2) {
        let curr: usize = (window[0] as usize) * WIDTH + (window[1] as usize);
        if map[curr] == 255 {
            continue;
        }
        map[curr] += 1;
    }

    let mut file: fs::File = fs::File::create(out_path).expect("Failed to create file");

    let _ = file.write_all(&format!("P6\n256 256\n255\n").as_bytes());

    let mut temp: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT * 3);

    map.iter().for_each(|v| {
        temp.push(*v);
        temp.push(*v);
        temp.push(*v)
    });

    let _ = file.write_all(&temp);
}
