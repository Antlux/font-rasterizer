use std::{fs::File, io::BufReader, path::Path};
use fontdue::{Font, Metrics};

pub type Rasterizations = Vec<(Metrics, Vec<u8>)>;

pub trait RasterManip {
}

impl RasterManip for Rasterizations {}


enum FontFaceError {
    FontOpeningError,
    CreationError(&'static str),
}

struct FontFace {
    font: Font,
}

impl FontFace {
    pub fn new(font_path: &Path) -> Result<Self, FontFaceError> {
        let file = File::open(font_path).map_err(|_| FontFaceError::FontOpeningError)?;
        let reader = BufReader::new(file);
        
        Ok(Self{ 
            font: {
                Font::from_bytes(
                    reader.buffer(),
                    fontdue::FontSettings::default()
                )
                .map_err(|err| FontFaceError::CreationError(err))?
            } 
        })
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


