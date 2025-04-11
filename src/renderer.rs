use std::{collections::HashMap, fmt::Display, fs::File, io::BufWriter};

use eframe::egui::ColorImage;
use fontdue::LineMetrics;

use crate::rasterization::{RasterizationProperty, Rasterizations};


#[derive(Debug)]
pub enum RendererError {
    InvalidPath,
    CreationError,
    TooLarge,
}

impl Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Path provided is invalid"),
            Self::CreationError => write!(f, "Encountered error creating render file"),
            Self::TooLarge => write!(f, "Texture too large"),
        }
    }
}

#[derive(Clone, Default)]
pub struct Padding {
    pub left: usize,
    pub right: usize,
    pub up: usize,
    pub down: usize,
}

impl Padding {
    pub fn horizontal(&self) -> usize {self.left + self.right}
    pub fn vertical(&self) -> usize {self.up + self.down}
}

#[derive(Clone)]
pub struct RenderSettings {
    pub input: Option<String>,
    pub render_height: f32,
    pub render_padding: Padding,
    pub render_layout: RenderLayout,
    pub render_direction: RenderDirection,
    pub sort_property: Option<RasterizationProperty>,
    pub dedup_property: Option<RasterizationProperty>,
    pub dedup_exact_duplicate: bool,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            input: None,
            render_height: 8.0,
            render_padding: Padding::default(),
            render_layout: RenderLayout::Squarish,
            render_direction: RenderDirection::LeftToRight,
            sort_property: Some(RasterizationProperty::Brightness),
            dedup_property: Some(RasterizationProperty::Brightness),
            dedup_exact_duplicate: true,
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum RenderLayout {
    Squarish,
    Horizontal,
    Vertical,
    Packed(bool),
    Custom(usize, usize)
}

impl RenderLayout {
    pub fn get_cell_counts(self, cell_count: usize, cell_width: usize, cell_height: usize) -> (usize, usize) {
        match self {
            RenderLayout::Squarish => {
                let total_pixels = cell_width * cell_height * cell_count;
                let target_width = (total_pixels as f32).sqrt().ceil();
                let h_count = (target_width / (cell_width as f32)).round();
                let v_count = (cell_count as f32 / h_count).ceil();
                (h_count as usize, v_count as usize)
            },
            RenderLayout::Horizontal => (cell_count, 1usize),
            RenderLayout::Vertical => (1usize, cell_count),
            RenderLayout::Packed(flipped) => {
                let mut map = HashMap::new();
                for i in 2..=(cell_count / 2) {
                    let remainder = cell_count % i;
                    let v = (i - remainder) % i;
    
                    map.insert(i, v);
                }
                
                let mut t = map
                .into_iter()
                .collect::<Vec<_>>();
                
                t.sort_by(|(_, a), (_, b)| b.cmp(a));
    
                let (_, rl) = t.last().unwrap_or(&(0, 0)).to_owned();
    
                t.retain(|(_, r)| r.to_owned() == rl.to_owned());
    
                t.sort_by(|(a, _), (b, _)| b.cmp(a));
    
                let mut h = cell_count;
                let mut distance = cell_count;
                let total_pixels = cell_width * cell_height * cell_count;
                let target_width = ((total_pixels as f32).sqrt() / (cell_width as f32)).round() as usize;
                for (d, _) in t {
                    let dist = (target_width).abs_diff(d);
                    if dist < distance {
                        h = d;
                        distance = dist;
                    }
                }
    
    
                let v = (cell_count as f32 / (h as f32)).ceil() as usize;
    
                if !flipped {
                    (h, v)
                } else {
                    (v, h)
                }
            },
            RenderLayout::Custom(h, v) => (h, v),
        }
    }
}

impl Display for RenderLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Squarish => write!(f, "Squarish"),
            Self::Horizontal => write!(f, "Horizontal"),
            Self::Vertical => write!(f, "Vertical"),
            Self::Packed(flipped) => {
                if flipped.to_owned() {
                    write!(f, "Packed (flipped)")
                } else {
                    write!(f, "Packed")
                }
            },
            Self::Custom(h, v) => write!(f, "Custom ({h} {v})")
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum RenderDirection {
    LeftToRight,
    TopToBottom,
}

impl Display for RenderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LeftToRight => write!(f, "Left to right"),
            Self::TopToBottom => write!(f, "Top to bottom"),
        }
    }
}

#[derive(Default, Clone)]
pub struct RenderData {
    width: usize,
    height: usize,
    pixels: Vec<u8>,
}

impl RenderData {
    pub fn renderable(&self) -> bool {
        self.width.max(self.height) <= 16384
    }
    pub fn width(&self) -> usize {self.width}
    pub fn height(&self) -> usize {self.height}
}

impl From<RenderData> for ColorImage {
    fn from(value: RenderData) -> Self {
        ColorImage::from_gray([value.width, value.height], &value.pixels)
    }
}


#[derive(Default, Clone)]
pub struct RenderInfo {
    cell_size: (usize, usize),
    cell_count: (usize, usize),
    cell_filled: usize,
    cell_padding: (usize, usize, usize, usize)
}

impl RenderInfo {
    pub fn cell_size(&self) -> (usize, usize) {self.cell_size}
    pub fn cell_count(&self) -> (usize, usize) {self.cell_count}
    pub fn cell_filled(&self) -> usize {self.cell_filled}
    pub fn cell_padding(&self) -> (usize, usize, usize, usize) {self.cell_padding}
}


pub fn generate_render_data(
    h_line_metrics: Option<LineMetrics>,
    v_line_metrics: Option<LineMetrics>,
    rasterizations: Rasterizations,
    render_settings: &RenderSettings
) -> (RenderData, RenderInfo) {

    let (vascent, vdescent) = if let Some(l_m) = h_line_metrics {
        (l_m.ascent, l_m.descent)
    } else {
        (
            rasterizations
                .iter()
                .map(|cr| cr.get_height())
                .max()
                .unwrap_or(render_settings.render_height.ceil() as usize) as f32,
            0.0
        )
    };

    let (hascent, hdescent) = if let Some(l_m) = v_line_metrics {
        (l_m.ascent, l_m.descent)
    } else {
        (
            rasterizations
                .iter()
                .map(|cr| cr.get_width())
                .max()
                .unwrap_or(render_settings.render_height.ceil() as usize) as f32,
            0.0
        )
    };

    let cell_width = (hascent - hdescent).round() as usize;
    let cell_height = (vascent - vdescent).round() as usize;

    let padded_cell_width = cell_width + render_settings.render_padding.horizontal();
    let padded_cell_height = cell_height + render_settings.render_padding.vertical();

    let raster_count = rasterizations.len();

    let (cell_h_count, cell_v_count) = render_settings.render_layout.get_cell_counts(
        raster_count, 
        padded_cell_width,
        padded_cell_height
    );

    let cell_count = cell_h_count * cell_v_count;

    let texture_width = cell_h_count * padded_cell_width;
    let texture_height = cell_v_count * padded_cell_height;

    let mut pixels = vec![0u8; texture_width * texture_height];

    for (idx, rasterization) in rasterizations.into_iter().enumerate() {

        let metrics = rasterization.get_metrics();
        let rasterization = rasterization.get_pixels();

        // let xmin = metrics.bounds.xmin.round() as isize;
        let ymin = metrics.bounds.ymin;

        let inverted_ymin = (vascent - ((metrics.height as f32) + ymin)).ceil() as isize;
        let width_offset = ((cell_width - metrics.width) as f32 / 2.0).ceil() as isize;

        let (cell_x, cell_y) = match render_settings.render_direction {
            RenderDirection::LeftToRight => {
                let x = idx % cell_h_count;
                let y = idx / cell_h_count;
                (x, y)
            },
            RenderDirection::TopToBottom => {
                let x = idx / cell_v_count;
                let y = idx % cell_v_count;
                (x, y)
            }
        };

        for (i, value) in rasterization.iter().enumerate() {
            // Pixel coordinate within character rasterization.
            let raster_relative_x = i % metrics.width;
            let raster_relative_y = (i - raster_relative_x) / metrics.width;

            // Pixel coordinate within cell.
            let cell_relative_x = raster_relative_x as isize + width_offset;
            let cell_relative_y = raster_relative_y as isize + inverted_ymin;

            // Absolute pixel coordinate within texture atlas.
            let x = ((cell_x * padded_cell_width) as isize
                + cell_relative_x)
                .max(0) as usize;
            let x = x + render_settings.render_padding.left; 

            let y = ((cell_y * padded_cell_height) as isize
                + cell_relative_y)
                .max(0) as usize;
            let y = y + render_settings.render_padding.up; 

            // Absolute pixel coordinate as index in pixel buffer.
            let index = x + (y * cell_h_count * padded_cell_width);
            if let Some(pixel) = pixels.get_mut(index) {
                *pixel = *value;
            } 
        }

    }

    (
        RenderData {
            width: texture_width,
            height: texture_height,
            pixels
        },
        RenderInfo {
            cell_count: (cell_h_count, cell_v_count),
            cell_size: (cell_width, cell_height),
            cell_filled: cell_count.min(raster_count),
            cell_padding: (
                render_settings.render_padding.left,
                render_settings.render_padding.right,
                render_settings.render_padding.up,
                render_settings.render_padding.down,
            )
        }
    )
}



pub fn write_image(name: String, render_data: &RenderData) -> Result<(), RendererError> {

    let width = render_data.width;
    let height = render_data.height;
    let pixels = &render_data.pixels;

    let render_path = rfd::FileDialog::new()
        .set_directory("/")
        .add_filter("png", &["png"])
        .set_file_name(name)
        .save_file()
        .ok_or(RendererError::InvalidPath)?;

    println!("{}", render_path.to_str().unwrap());

    println!("Trying to create file at {}", render_path.display());

    let file = File::create(render_path).map_err(|_| RendererError::CreationError)?;
    let ref mut writer = BufWriter::new(file);

    let mut encoder = png::Encoder::new(writer, width as u32, height as u32);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(
        // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000),
    );
    encoder.set_source_chromaticities(source_chromaticities);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(pixels).unwrap();

    Ok(())
}
