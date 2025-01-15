use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

use fontdue::Font;

use crate::rasterization::Rasterizations;

#[derive(Debug)]
pub enum FontFaceError {
    FontOpeningError,
    CreationError(&'static str),
}

impl Display for FontFaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FontOpeningError => write!(f, "Encountered error opening font file."),
            Self::CreationError(err) => write!(f, "Encountered error creating font face: {err}"),
        }
    }
}

pub struct FontFace {
    font: Font,
    path: PathBuf,
}

impl FontFace {
    pub fn load(font_path: PathBuf) -> Result<Self, FontFaceError> {
        let mut buf = vec![];
        let mut file = File::open(&font_path).map_err(|_| FontFaceError::FontOpeningError)?;
        let _ = file.read_to_end(&mut buf);

        Ok(Self {
            font: {
                Font::from_bytes(buf, fontdue::FontSettings::default())
                    .map_err(|err| FontFaceError::CreationError(err))?
            },
            path: font_path,
        })
    }

    pub fn stem(&self) -> &str {
        self.path.file_stem().unwrap().to_str().unwrap().into()
    }

    pub fn path(&self) -> &str {
        self.path.to_str().unwrap()
    }

    pub fn chars(&self) -> Vec<char> {
        self.font.chars().keys().map(|c| *c).collect::<Vec<_>>()
    }

    pub fn rasterize(&self, input: Option<Vec<char>>, pixel_height: f32) -> Rasterizations {
        let chars = input.unwrap_or(self.chars());

        chars
            .iter()
            .map(|c| self.font.rasterize(*c, pixel_height))
            .collect::<Vec<_>>() as Rasterizations
    }
}