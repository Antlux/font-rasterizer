use std::{fmt::Display, str::FromStr};

use rasterization::{FontFace, RasterManip, RasterizationSort};

pub mod subcommands;
pub mod rasterization;


pub enum GenerationError {
    LayoutParsingError
}


pub enum GenerationLayout {
    Rect,
    Linear,
}

impl GenerationLayout {
    pub fn keys() -> Vec<String> {
        vec![
            GenerationLayout::Rect.to_string(),
            GenerationLayout::Linear.to_string(),
        ]
    }
}


impl FromStr for GenerationLayout {
    type Err = GenerationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "rect" => Ok(GenerationLayout::Rect),
            "linear" => Ok(GenerationLayout::Linear),
            _ => Err(GenerationError::LayoutParsingError),
        }
    }
}

impl Display for GenerationLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Linear => write!(f, "linear"),
            Self::Rect => write!(f, "rect"),
        }
    }
}



pub fn generate_gradient(
    font_face: FontFace,
    input: Option<Vec<char>>,
    cell_width: usize,
    cell_height: usize,
    pixel_height: f32,
    layout: GenerationLayout
) -> Vec<u8> {
    let mut rasterizations = font_face.rasterize(input, pixel_height);
    rasterizations.sort_rasters_by(RasterizationSort::Brightness);
    
    let cell_h_count;
    let cell_v_count;

    match layout {
        GenerationLayout::Linear => {
            cell_h_count = rasterizations.len();
            cell_v_count = 1usize;
        },
        GenerationLayout::Rect => {
            cell_h_count = (rasterizations.len() as f32).sqrt().ceil() as usize;
            cell_v_count = ((rasterizations.len() as f32) / (cell_h_count as f32)).ceil() as usize;
        },
    }

    let mut data_out = vec![0u8; cell_width * cell_h_count * cell_height * cell_v_count];

    for (i, (metrics, rasterization)) in rasterizations.iter().enumerate() {
        let x = i % cell_h_count;
        let y = (i - x) / cell_h_count;
        for (i, value) in rasterization.iter().enumerate() {
            let a = i % metrics.width;
            let b = (i - a) % metrics.width;
            let x = x + a;
            let y = y + b;
            let index = x + y * cell_h_count * cell_width;
            if let Some(pixel) = data_out.get_mut(index) {
                *pixel = *value;
            }
        }
    }

    data_out
}