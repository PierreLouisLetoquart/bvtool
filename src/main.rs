use bvtool::{generate_visualization, load_file, map_to_image, Map256};
use clap::{Parser, ValueEnum};
use image::RgbImage;
use serde::Serialize;

// Possibilities
//
// generate a heatmap of the entire file
// save the heatmap as a .png file (default to data/heatmap.png)
// save the heatmap as a .png file in a custom path
// specify the color of the heatmap (default to white)
// generate a heatmap for all the files in a directory

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Colors {
    #[default]
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: Option<std::path::PathBuf>,

    #[arg(short, long, default_value = "data/heatmap.png")]
    output: std::path::PathBuf,

    #[arg(short, long)]
    dir: Option<String>,

    #[arg(short, long, default_value_t, value_enum)]
    color: Colors,

    #[arg(short, long, default_value = "false")]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    if let Some(path) = args.path {
        println!("Loading file: {:?}", path);
    } else if let Some(dir) = args.dir {
        println!("Loading directory: {:?}", dir);
    } else {
        eprintln!("No path or directory specified");
        println!("run `bvtool --help` for more information");
        std::process::exit(1);
    }

    // get the path of all the files in "path"
    // let mut limit = 0;
    // let paths = std::fs::read_dir("").unwrap();
    // for entry in paths {
    //     let entry = entry.unwrap();
    //     let file_path = entry.path();
    //     let file_name = file_path.file_name().unwrap().to_str().unwrap();

    //     println!("[LOADING] {}", file_name);
    //     let slice = load_file(&file_path.to_str().unwrap());
    //     let mut map = Map256::zeros();

    //     println!("[GENERATING] generating 256x256 map");
    //     generate_visualization(&slice, &mut map);
    //     let img: RgbImage = map_to_image(&map);

    //     let out_path = format!("data/img-jpg/{}.bvtool.png", file_name);
    //     println!("[DONE] saving map into {}", &out_path);
    //     img.save(out_path).unwrap();
    //     limit += 1;
    //     if limit == 8000 {
    //         // finish program
    //         break;
    //     }

    // // Open target file
    // let file_path = "./target/debug/bvtool";
    // let slice = load_file(&file_path);

    // // Create the 256x256 map
    // let mut map = Map256::zeros();

    // // Generate the visualization
    // generate_visualization(&slice, &mut map);

    // // Create the image
    // let img: RgbImage = map_to_image(&map);

    // // Save the image
    // img.save("data/heatmap.png").unwrap();
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
            let mut map = Map256::zeros();

            println!("[GENERATING] generating 256x256 map");
            generate_visualization(&slice, &mut map);
            let img: RgbImage = map_to_image(&map);

            let out_path = format!("data/exec/{}.bvtool.png", file_name);
            println!("[DONE] saving map into {}", &out_path);
            img.save(out_path).unwrap();
        }
    }
}
