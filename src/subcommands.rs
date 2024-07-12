use std::{fmt::{Debug, Display}, io::stdin, str::FromStr};

pub enum SubcommandError {
    NoPath,
    InvalidPath,
    MissingCellDim,
    InputParsingError,
}

impl Display for SubcommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPath => write!(f, "No path was provided."),
            Self::InvalidPath => write!(f, "Invalid Path."),
            Self::MissingCellDim => write!(f, "Must provide dimension."),
            Self::InputParsingError => write!(f, "Encountered error parsing user input.")
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
    let cell_width;
    loop {
        if let Some(width) = get_input::<usize>().ok() {
            cell_width = width;
            break;
        } else {
            println!("Error parsing input. Please enter cell height (in whole pixels):");
        }
    }
    
    println!("Please enter cell width (in whole pixels):");
    let cell_height;
    loop {
        if let Some(height) = get_input::<usize>().ok() {
            cell_height = height;
            break;
        } else {
            println!("Error parsing input. Please enter cell height (in whole pixels):");
        }
    }


    println!("Please enter font render height:");
    let pixel_height;
    loop {
        if let Some(height) = get_input::<f32>().ok() {
            pixel_height = height;
            break;
        } else {
            println!("Error parsing input. Please enter cell height (in whole pixels):");
        }
    }


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