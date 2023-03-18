use std::env;

use crate::{
    logger::{LogType, Path},
    settings::Settings,
};

mod cropper;
mod logger;
mod settings;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!(
            "Usage: {} <input-image> <width:height> <rows:cols>",
            args[0]
        );
        return;
    }

    LogType::SettingsInit.log();

    let settings = Settings::init(args);

    LogType::ImageResize.log();

    let resized = settings.img.resize_exact(
        settings.col_width * settings.num_cols,
        settings.row_height * settings.num_rows,
        image::imageops::FilterType::Lanczos3,
    );

    LogType::OutputInit(Path::from(&settings.path)).log();
    std::fs::create_dir_all(&settings.path).unwrap();
    let total = settings.num_rows * settings.num_cols;

    for i in 0..settings.num_rows {
        for j in 0..settings.num_cols {
            cropper::crop(&settings, &resized, i, j);
            logger::print_progress(i * settings.num_cols + j + 1, total);
        }
    }

    LogType::Finished(Path::from(&settings.path)).log();
}
