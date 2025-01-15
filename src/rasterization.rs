use fontdue::{Font, Metrics};
use std::{collections::{HashMap, HashSet}, fmt::Display, fs::File, io::Read, path::PathBuf};

#[derive(Clone, Copy, PartialEq)]
pub enum RasterizationProperty {
    Brightness,
    Width,
    Height,
}

impl Display for RasterizationProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Brightness => write!(f, "Brightness"),
            Self::Width => write!(f, "Width"),
            Self::Height => write!(f, "Height"),
        }
    }
}


pub type CharRaster = (Metrics, Vec<u8>);
pub trait RasterInfo {
    fn get_property(&self, property: RasterizationProperty) -> usize;
    fn get_brightness(&self) -> usize;
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

impl RasterInfo for CharRaster {
    fn get_property(&self, property: RasterizationProperty) -> usize {
        match property {
            RasterizationProperty::Brightness => self.get_brightness(),
            RasterizationProperty::Width => self.get_width(),
            RasterizationProperty::Height => self.get_height(),
        }
    }

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

pub type Rasterizations = Vec<CharRaster>;
pub trait RasterManip {
    fn count_duplicates(&self, property: RasterizationProperty) -> usize;
    fn sort_rasters_by(&mut self, property: RasterizationProperty);
    fn dedup_rasters_by(&mut self, property: RasterizationProperty);
    fn dedup_exact_duplicate(&mut self);
}

impl RasterManip for Rasterizations {
    fn count_duplicates(&self, property: RasterizationProperty) -> usize {
        let mut counter: HashMap<usize, usize> = HashMap::new();
        for raster in self {
            let value = raster.get_property(property);
            if let Some(count) = counter.get_mut(&value) {
                *count += 1;
            } else {
                counter.insert(value, 1);
            }
        }
        counter.values().into_iter().map(|e| e - 1).sum::<usize>()
    }

    fn sort_rasters_by(&mut self, property: RasterizationProperty) {
        self.sort_by(|a, b| a.get_property(property).cmp(&b.get_property(property)));
    }
    
    fn dedup_rasters_by(&mut self, property: RasterizationProperty) {

        let mut set = HashSet::new();
        self.retain(|r| set.insert(r.get_property(property)));
    }

    fn dedup_exact_duplicate(&mut self) {
        let mut set = HashSet::new();
        self.retain(|(_, p)| set.insert(p.clone()));
    }
}

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
