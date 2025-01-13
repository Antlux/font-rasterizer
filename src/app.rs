use std::{
    fmt::{Debug, Display},
    io::stdin,
    str::FromStr,
};

use eframe::egui;

use crate::{
    rasterization::{FontFace, FontFaceError},
    renderer::RendererError,
};


#[derive(Default)]
pub struct FontRasterizerApp {
    font_face: Option<FontFace>
}

impl FontRasterizerApp {
    fn load_font(&mut self) {
        self.font_face = get_font_face().ok();
    }
}

impl eframe::App for FontRasterizerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui|{
                ui.heading("Font Rasterizer");
                ui.separator();
                ui.label("Font:");

                let button_text = if let Some(font_face) = &self.font_face {
                    font_face.stem()
                } else {
                    "Load"
                };

                if ui.button(button_text).clicked() {
                    self.load_font();
                }
            });
            ui.separator();
        });
        // egui::SidePanel::right("job-history").show(ctx, |ui| {
        //     ui.label("Job History");
        // });
    }
}








#[derive(Debug)]
pub enum AppError {
    NoFontPath,
    InvalidFontPath,
    FontLoadingError(FontFaceError),
    MissingCellDim,
    InputParsingError,
    RenderingError(RendererError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFontPath => write!(f, "No path was provided."),
            Self::InvalidFontPath => write!(f, "Invalid Path."),
            Self::FontLoadingError(err) => write!(f, "Encountered error loading font: {err}."),
            Self::MissingCellDim => write!(f, "Must provide dimension."),
            Self::InputParsingError => write!(f, "Encountered error parsing user input."),
            Self::RenderingError(err) => write!(f, "Encountered error rendering: {err}."),
        }
    }
}

pub fn get_font_face() -> Result<FontFace, AppError> {
    let font_path = rfd::FileDialog::new()
        .add_filter("font", &["ttf", "ttc", "otf"])
        .set_directory("/")
        .pick_file()
        .ok_or(AppError::NoFontPath)?;
    FontFace::load(font_path).map_err(|err| AppError::FontLoadingError(err))
}

pub fn get_input<T>() -> Result<T, <T as FromStr>::Err>
where
    T: ToString + FromStr,
    <T as FromStr>::Err: Debug,
{
    let mut input_buf = String::new();
    let _ = stdin().read_line(&mut input_buf);
    input_buf.trim().split(' ').next().unwrap().parse::<T>()
}
