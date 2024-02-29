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
}
