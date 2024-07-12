use std::{fmt::Display, io::Read, num::ParseFloatError, path::Path};

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
    println!("Please provide arguments as so:");
    println!("<font_path> <cell_width> <cell_height> [char_height (pixels: f32)]");

    let mut user_input = "".into(); 
    std::io::stdin().read_line(&mut user_input).unwrap();
    let mut args = user_input.trim().split(' ');

    let font_path = Path::new(
        args.next().ok_or(SubcommandError::NoPath)
        .and_then(|path| {
            if !path.is_empty() {
                Ok(path)
            } else {
                Err(SubcommandError::NoPath)
            }
        })?
    );

    font_path
        .try_exists()
        .map_err(|_| SubcommandError::InvalidPath)
        .and_then(|exists| {
            if !exists {Err(SubcommandError::InvalidPath)} else {Ok(())}
        })?;

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

    println!("{} - {} {} - {}px", font_path.display(), cell_width, cell_height, pixel_height);

    Ok(())

    println!("Please select a layout:")

}

pub fn sequence() {

}

pub fn variants() {

}