use std::{
    fmt::{Debug, Display},
    io::stdin,
    str::FromStr,
};

use crate::{
    rasterization::{FontFace, FontFaceError},
    renderer::RendererError,
};

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
