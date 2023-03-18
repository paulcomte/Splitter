use image::DynamicImage;

use crate::Settings;

pub fn crop(settings: &Settings, resized: &DynamicImage, i: u32, j: u32) {
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
    imgbuf
        .save_with_format(output_path, image::ImageFormat::Png)
        .unwrap();
}
