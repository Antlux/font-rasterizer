use std::{fmt::Display, num::ParseFloatError};

pub enum SubcommandError {
    NoPath,
    InvalidPath,
    MissingCellDim,
    CellDimParsingError(ParseFloatError),
}

impl Display for SubcommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPath => write!(f, "No path was provided."),
            Self::InvalidPath => write!(f, "Invalid Path."),
            Self::MissingCellDim => write!(f, "Must provide dimension."),
            Self::CellDimParsingError(err) => {
                write!(f, "Encountered error parsing cell dimension: {err}.")
            }
        }
    }
}

pub fn gradient() -> Result<(), SubcommandError> {

    let file = rfd::FileDialog::new()
        .add_filter("font", &["ttf", "ttc", "otf"])
        .set_directory("/")
        .pick_file()
        .ok_or(SubcommandError::NoPath)?;




    println!("Please provide arguments as so:");
    println!("<font_path> <cell_width> <cell_height> [char_height (pixels: f32)]");

    let mut user_input = "".into(); 
    std::io::stdin().read_line(&mut user_input).unwrap();
    let mut args = user_input.trim().split(' ');

    let cell_width = args
        .next()
        .ok_or(SubcommandError::MissingCellDim)?
        .parse::<f32>()
        .map_err(|err| SubcommandError::CellDimParsingError(err))?;

    let cell_height = args
        .next()
        .ok_or(SubcommandError::MissingCellDim)?
        .parse::<f32>()
        .map_err(|err| SubcommandError::CellDimParsingError(err))?;

    let pixel_height = if let Some(arg) = args.next() {
        arg.parse::<f32>().map_err(|err| SubcommandError::CellDimParsingError(err))?
    } else {
        cell_height
    };


    Ok(())
}

pub fn sequence() {

}

pub fn variants() {

}