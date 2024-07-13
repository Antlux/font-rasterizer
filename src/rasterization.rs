use std::{fmt::Display, fs::File, io::Read, path::Path};
use fontdue::{Font, Metrics};
pub enum RasterizationSort {
    Brightness,
}

pub type Rasterization = (Metrics, Vec<u8>);
trait RasterInfo {
    fn brightness(&self) -> usize;
}

impl RasterInfo for Rasterization {
    fn brightness(&self) -> usize {
        let (_, data) = &self;
        data
            .into_iter()
            .map(|v| v.to_owned() as usize)
            .sum()
    }
}



pub type Rasterizations = Vec<Rasterization>;
pub trait RasterManip {
    fn sort_rasters_by(&mut self, sort: RasterizationSort);
}

impl RasterManip for Rasterizations {
    fn sort_rasters_by(&mut self, sort: RasterizationSort) {
        match sort {
            RasterizationSort::Brightness => {
                self.sort_by(|a, b| {
                    a.brightness().cmp(&b.brightness())
                });
            }
        }
    }
}


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
    name: String,
}

impl FontFace {
    pub fn load(font_path: &Path) -> Result<Self, FontFaceError> {
        let mut buf = vec![]; 
        let mut file = File::open(font_path).map_err(|_| FontFaceError::FontOpeningError)?;
        let _ = file.read_to_end(&mut buf);

        Ok(Self{ 
            font: {
                Font::from_bytes(
                    buf,
                    fontdue::FontSettings::default()
                )
                .map_err(|err| FontFaceError::CreationError(err))?
            },
            name: font_path.file_name().unwrap().to_str().unwrap().into()
        })
    }

    pub fn name(&self) -> &str {
        &self.name
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


