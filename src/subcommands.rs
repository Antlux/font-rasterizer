use std::{fmt::{Debug, Display}, io::stdin, num::ParseFloatError, str::FromStr};

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


pub fn get_input<T>() -> Result<T, <T as FromStr>::Err> where T: ToString + FromStr, <T as FromStr>::Err: Debug {
    let mut input_buf = String::new();
    let _ = stdin().read_line(&mut input_buf);
    input_buf
        .trim()
        .split(' ')
        .next()
        .unwrap()
        .parse::<T>()
}


pub fn gradient() -> Result<(), SubcommandError> {
    let file_path = rfd::FileDialog::new()
        .add_filter("font", &["ttf", "ttc", "otf"])
        .set_directory("/")
        .pick_file()
        .ok_or(SubcommandError::NoPath)?;

    println!("Please enter cell height (in whole pixels):");
    let cell_width = get_input::<usize>().unwrap_or(8);
    
    println!("Please enter cell width (in whole pixels):");
    let cell_height = get_input::<usize>().unwrap_or(8);

    println!("Please enter font render height:");
    let pixel_height = get_input::<f32>().unwrap_or(8.0);

    println!("Rendering gradient from {} with cells of {} by {} pixels rendered at {}", 
        file_path.file_name().unwrap().to_str().unwrap(),
        cell_width, 
        cell_height, 
        pixel_height
    );

    Ok(())
}

pub fn sequence() {

}

pub fn variants() {

}