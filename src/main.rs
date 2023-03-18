use image::{DynamicImage, GenericImageView};
use std::env;
use std::io::{stdout, Write};
use std::path::Path;

struct Settings {
    path: String,
    img: DynamicImage,
    width: u32,
    height: u32,
    num_rows: u32,
    num_cols: u32,
    row_height: u32,
    col_width: u32,
}

impl Settings {
    fn init(args: Vec<String>) -> Self {
        let input_path = Path::new(&args[1]);
        let output_dimensions: Vec<_> = args[2]
            .split(':')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let output_num: Vec<_> = args[3]
            .split(':')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let img = image::open(input_path).unwrap();
        let (img_width, img_height) = img.dimensions();

        Self {
            path: input_path.file_stem().unwrap().to_str().unwrap().into(),
            img,
            width: output_dimensions[0],
            height: output_dimensions[1],
            num_rows: output_num[0],
            num_cols: output_num[1],
            row_height: img_height / output_num[0],
            col_width: img_width / output_num[1],
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <input-image> <width:height> <rows:cols>",
            args[0]
        );
        return;
    }
    println!("[Splitter]: Initializing environment.");

    let settings = Settings::init(args);

    println!("[Splitter]: Resizing image.");

    let resized = settings.img.resize_exact(
        settings.col_width * settings.num_cols,
        settings.row_height * settings.num_rows,
        image::imageops::FilterType::Lanczos3,
    );

    std::fs::create_dir_all(&settings.path).unwrap();
    let total = settings.num_rows * settings.num_cols;

    for i in 0..settings.num_rows {
        for j in 0..settings.num_cols {
            let row_start = i * settings.row_height;
            let col_start = j * settings.col_width;

            let imgbuf = DynamicImage::ImageRgb8(resized.to_rgb8())
                .crop(
                    col_start,
                    row_start,
                    settings.col_width,
                    settings.row_height,
                )
                .resize_exact(
                    settings.width,
                    settings.height,
                    image::imageops::FilterType::Lanczos3,
                )
                .to_rgb8();

            let output_path = format!("{}/{}_{}.png", settings.path, i, j);
            imgbuf.save(output_path).unwrap();
            print_progress(i * settings.num_cols + j + 1, total);
        }
    }
    println!("\n[Splitter]: Images saved in: [{}/]", settings.path);
}

fn print_progress(current: u32, total: u32) {
    let progress = current as f32 / total as f32;
    let bar_length = 20;
    let filled_length = (progress * bar_length as f32).round() as usize;
    let empty_length = bar_length - filled_length;
    let animation = match current % 4 {
        0 => "|",
        1 => "/",
        2 => "-",
        _ => "\\",
    };
    // Print the progress bar
    print!(
        "\r[{}{}] {:.0}% {}",
        "=".repeat(filled_length),
        " ".repeat(empty_length),
        (progress * 100.0),
        animation,
    );

    // Flush the output buffer to make sure it's immediately displayed on the console
    let _ = stdout().flush();
}
