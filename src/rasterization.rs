use fontdue::{Font, Metrics};
use std::{fmt::Display, fs::File, io::Read, path::Path};
pub enum RasterizationProperty {
    Brightness,
    Width,
    Height,
}

pub type Rasterization = (Metrics, Vec<u8>);
pub trait RasterInfo {
    fn get_brightness(&self) -> usize;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

impl RasterInfo for Rasterization {
    fn get_brightness(&self) -> usize {
        let (_, data) = &self;
        data.into_iter().map(|v| v.to_owned() as usize).sum()
    }
    fn get_width(&self) -> usize {
        let (m, _) = self;
        m.height
    }
    fn get_height(&self) -> usize {
        let (m, _) = self;
        m.height
    }
}

pub type Rasterizations = Vec<Rasterization>;
pub trait RasterManip {
    fn sort_rasters_by(&mut self, property: RasterizationProperty);
    fn dedup_rasters_by(&mut self, property: RasterizationProperty);
}

impl RasterManip for Rasterizations {
    fn sort_rasters_by(&mut self, property: RasterizationProperty) {
        match property {
            RasterizationProperty::Brightness => {
                self.sort_by(|a, b| a.get_brightness().cmp(&b.get_brightness()));
            }
            _ => {}
        }
    }
    fn dedup_rasters_by(&mut self, property: RasterizationProperty) {
        match property {
            RasterizationProperty::Brightness => {
                self.dedup_by(|a, b| a.get_brightness() == b.get_brightness())
            }
            RasterizationProperty::Width => self.dedup_by(|a, b| a.get_width() == b.get_width()),
            RasterizationProperty::Height => self.dedup_by(|a, b| a.get_height() == b.get_height()),
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

        Ok(Self {
            font: {
                Font::from_bytes(buf, fontdue::FontSettings::default())
                    .map_err(|err| FontFaceError::CreationError(err))?
            },
            name: font_path.file_name().unwrap().to_str().unwrap().into(),
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
