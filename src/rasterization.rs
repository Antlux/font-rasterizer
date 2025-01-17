use fontdue::Metrics;
use std::{collections::{HashMap, HashSet}, fmt::Display};

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

pub struct CharRaster {
    metrics: Metrics,
    pixels: Vec<u8>,
}

impl CharRaster {
    pub fn new((metrics, pixels): (Metrics, Vec<u8>)) -> Self {
        Self {
            metrics,
            pixels
        }
    }

    fn get_property(&self, property: RasterizationProperty) -> usize {
        match property {
            RasterizationProperty::Brightness => self.get_brightness(),
            RasterizationProperty::Width => self.get_width(),
            RasterizationProperty::Height => self.get_height(),
        }
    }

    pub fn get_metrics(&self) -> Metrics {
        self.metrics
    }

    pub fn get_pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }

    pub fn get_brightness(&self) -> usize {
        self.pixels.iter().map(|v| *v as usize).sum()
    }
    
    pub fn get_width(&self) -> usize {
        self.metrics.width
    }
    
    pub fn get_height(&self) -> usize {
        self.metrics.height
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
        self.retain(|cr| set.insert(cr.pixels.clone()));
    }
}


