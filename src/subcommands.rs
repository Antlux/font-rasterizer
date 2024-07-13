use std::{fmt::{Debug, Display}, io::stdin, str::FromStr};

use dialoguer::Select;

use crate::{generate_gradient, rasterization::{FontFace, FontFaceError}, renderer::{render, Image, RendererError}, GenerationLayout};

pub enum SubcommandError {
    NoFontPath,
    InvalidFontPath,
    FontLoadingError(FontFaceError),
    MissingCellDim,
    InputParsingError,
    RenderingError(RendererError)
}

impl Display for SubcommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFontPath => write!(f, "No path was provided."),
            Self::InvalidFontPath => write!(f, "Invalid Path."),
            Self::FontLoadingError(err) => write!(f, "Encountered error loading font: {err}."),
            Self::MissingCellDim => write!(f, "Must provide dimension."),
            Self::InputParsingError => write!(f, "Encountered error parsing user input."),
            Self::RenderingError(err) => write!(f, "Encountered error rendering: {err}.")
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
    let font_path = rfd::FileDialog::new()
        .add_filter("font", &["ttf", "ttc", "otf"])
        .set_directory("/")
        .pick_file()
        .ok_or(SubcommandError::NoFontPath)?;

    let font_face = FontFace::load(font_path.as_path()).map_err(|err| SubcommandError::FontLoadingError(err))?;

    println!("Enter cell height (in whole pixels):");
    let cell_width;
    loop {
        if let Some(width) = get_input::<usize>().ok() {
            cell_width = width;
            break;
        } else {
            println!("Error parsing input. Please enter cell height (in whole pixels):");
        }
    }
    
    println!("Enter cell width (in whole pixels):");
    let cell_height;
    loop {
        if let Some(height) = get_input::<usize>().ok() {
            cell_height = height;
            break;
        } else {
            println!("Error parsing input. Please enter cell height (in whole pixels):");
        }
    }


    println!("Enter font render height:");
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
        font_face.name(),
        cell_width, 
        cell_height, 
        pixel_height
    );

    let layouts = GenerationLayout::keys();
    let layout;
    loop {
        let layout_selection = Select::new()
        .with_prompt("Choose render layout")
        .items(&layouts)
        .interact()
        .unwrap();

        if let Some(l) = layouts.get(layout_selection) {
            if let Some(l) = l.parse().ok() {
                layout = l;
                break;
            }
        }
    }
    
    let (width, height, data) = generate_gradient(&font_face, None, cell_width, cell_height, pixel_height, layout, true);
    let image = Image::Grayscale(font_face.name().to_owned(), width, height, data);

    render(image).map_err(|err| SubcommandError::RenderingError(err))?;

    Ok(())
}


// let selection = Select::new()
//         .with_prompt("Choose generation type")
//         .items(&subcommands)
//         .interact()
//         .unwrap();

//     match subcommands[selection] {
//         "gradient" => gradient().unwrap_or_else(|err| eprintln!("ERROR: {err}")),
//         "sequence" => sequence(),
//         "variants" => variants(),
//         "help" => help(),
//         other => {
//             println!("'{other}' is not a recognized subcommand.");
//             help();
//         }         
//     }


pub fn sequence() {

}

pub fn variants() {

}