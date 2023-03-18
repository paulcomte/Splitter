use std::path::Path;

use image::{DynamicImage, GenericImageView};

pub struct Settings {
    pub path: String,
    pub img: DynamicImage,
    pub width: u32,
    pub height: u32,
    pub num_rows: u32,
    pub num_cols: u32,
    pub row_height: u32,
    pub col_width: u32,
}

impl Settings {
    pub fn init(args: Vec<String>) -> Self {
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
