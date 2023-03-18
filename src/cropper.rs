use image::DynamicImage;

use crate::{mapper::rgb_to_short, Settings};

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

    let mut output = Vec::new();
    for pixel in imgbuf.pixels() {
        let xterm_color = rgb_to_short(pixel.0[0], pixel.0[1], pixel.0[2]);
        output.push(xterm_color[0]);
        output.push(xterm_color[1]);
        output.push(xterm_color[2]);
    }

    let output_path = format!("{}/{}_{}.png", settings.path, i, j);
    image::save_buffer_with_format(
        output_path,
        &output,
        settings.width,
        settings.height,
        image::ColorType::Rgb8,
        image::ImageFormat::Png,
    )
    .unwrap();
}
