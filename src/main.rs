use std::env;
use std::path::Path;

use image::{DynamicImage, GenericImageView, RgbaImage};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input-image> <rows:cols>", args[0]);
        return;
    }

    let input_path = Path::new(&args[1]);
    let output_dimensions: Vec<_> = args[2]
        .split(':')
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    let img = image::open(input_path).unwrap();

    let (img_width, img_height) = img.dimensions();
    let num_rows = output_dimensions[0];
    let num_cols = output_dimensions[1];
    let row_height = img_height / num_rows;
    let col_width = img_width / num_cols;
    for i in 0..num_rows {
        for j in 0..num_cols {
            let row_start = i * row_height;
            let col_start = j * col_width;

            let imgbuf = DynamicImage::ImageRgb8(img.to_rgb8())
                .crop(col_start, row_start, col_width, row_height)
                .to_rgb8();

            let output_path = format!(
                "{}_{}_{}.png",
                input_path.file_stem().unwrap().to_str().unwrap(),
                i,
                j
            );
            imgbuf.save(output_path).unwrap();
        }
    }
}
