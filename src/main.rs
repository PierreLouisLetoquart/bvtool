use bvtool::{generate_visualization, load_file, map_to_image, Map256};
use clap::Parser;
use image::RgbImage;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: std::path::PathBuf,

    #[arg(short, long, default_value = "data/heatmap.png")]
    output: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let slice = load_file(&args.path);
    let mut map = Map256::zeros();

    generate_visualization(&slice, &mut map);
    let img: RgbImage = map_to_image(&map);

    img.save(&args.output).unwrap();
    // recursive();
}

// fn recursive() {
//     let dir = "/Users/pilou/Coding/binviz/bvtool/data/speech/Actor_01";
//     let dir_out = "/Users/pilou/Coding/binviz/bvtool/data/speech";
//
//     let files = std::fs::read_dir(dir).unwrap();
//
//     for file in files {
//         let file = file.unwrap();
//         let path = file.path();
//         let name = path.file_name().unwrap().to_str().unwrap();
//         let out = format!("{}/actor_1_{}.png", dir_out, name);
//
//         let slice = load_file(&path);
//         let mut map = Map256::zeros();
//
//         generate_visualization(&slice, &mut map);
//         let img: RgbImage = map_to_image(&map);
//
//         img.save(&out).unwrap();
//     }
// }
