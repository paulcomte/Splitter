use std::io::{stdout, Write};

pub struct Path(String);

impl From<&String> for Path {
    fn from(str: &String) -> Self {
        Self(str.into())
    }
}

pub enum LogType {
    SettingsInit,
    ImageResize,
    OutputInit(Path),
    Finished(Path),
}

impl LogType {
    pub fn log(&self) {
        match self {
            Self::SettingsInit => self.fast_log("Initializing environment."),
            Self::ImageResize => self.fast_log("Resizing original image."),
            Self::OutputInit(path) => self.fast_log(&format!("Creating directory: [{}/]", path.0)),
            Self::Finished(path) => {
                println!("");
                self.fast_log(&format!("Images saved in: [{}/] folder.", path.0))
            }
        }
    }

    fn fast_log(&self, message: &str) {
        println!("[Splitter]: {message}");
    }
}

pub fn print_progress(current: u32, total: u32) {
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
