use std::{fmt::Display, str::FromStr};

use rasterization::{FontFace, RasterManip, RasterizationProperty};

pub mod subcommands;
pub mod rasterization;
pub mod renderer;


pub enum GenerationError {
    LayoutParsingError
}


pub enum GenerationLayout {
    Square,
    Horizontal,
    Vertical,
}

impl GenerationLayout {
    pub fn keys() -> Vec<String> {
        vec![
            GenerationLayout::Square.to_string(),
            GenerationLayout::Horizontal.to_string(),
            GenerationLayout::Vertical.to_string(),
        ]
    }
}


impl FromStr for GenerationLayout {
    type Err = GenerationError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "square" => Ok(GenerationLayout::Square),
            "horizontal" => Ok(GenerationLayout::Horizontal),
            "vertical" => Ok(GenerationLayout::Vertical),
            _ => Err(GenerationError::LayoutParsingError),
        }
    }
}

impl Display for GenerationLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Horizontal => write!(f, "linear"),
            Self::Vertical => write!(f, "horizontal"),
            Self::Square => write!(f, "square"),
        }
    }
}



pub fn generate_gradient(
    font_face: &FontFace,
    input: Option<Vec<char>>,
    cell_width: usize,
    cell_height: usize,
    pixel_height: f32,
    layout: GenerationLayout,
    dedup: bool
) -> (usize, usize, Vec<u8>) {
    let mut rasterizations = font_face.rasterize(input, pixel_height);
    rasterizations.sort_rasters_by(RasterizationProperty::Brightness);
    if dedup {rasterizations.dedup_rasters_by(RasterizationProperty::Brightness)}
    
    let cell_h_count;
    let cell_v_count;

    match layout {
        GenerationLayout::Horizontal => {
            cell_h_count = rasterizations.len();
            cell_v_count = 1usize;
        },
        GenerationLayout::Vertical => {
            cell_h_count = 1usize;
            cell_v_count = rasterizations.len();
        }
        GenerationLayout::Square => {
            cell_h_count = (rasterizations.len() as f32).sqrt().ceil() as usize;
            cell_v_count = ((rasterizations.len() as f32) / (cell_h_count as f32)).ceil() as usize;
        },
    }

    let mut data_out = vec![0u8; cell_width * cell_h_count * cell_height * cell_v_count];

    for (i, (metrics, rasterization)) in rasterizations.iter().enumerate() {
        let cell_pos_x = i % cell_h_count;
        let cell_pos_y = (i - cell_pos_x) / cell_h_count;
        for (i, value) in rasterization.iter().enumerate() {
            let cell_relative_x = i % metrics.width;
            let cell_relative_y = (i - cell_relative_x) / metrics.width;
            let x = cell_pos_x * cell_width + cell_relative_x;
            let y = cell_pos_y * cell_height + cell_relative_y;
            let x = x + ((cell_width - metrics.width) / 2).max(0);
            let y = y + ((cell_height - metrics.height) / 2).max(0);
            let index = x + (y * cell_h_count * cell_width);
            if let Some(pixel) = data_out.get_mut(index) {
                *pixel = *value;
            }
        }
    }

    (cell_h_count * cell_width, cell_v_count * cell_height, data_out)
}