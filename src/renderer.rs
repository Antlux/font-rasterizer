use std::{fmt::Display, fs::File, io::BufWriter};

use eframe::egui::ColorImage;

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

#[derive(Clone)]
pub struct RenderSettings {
    pub render_height: f32,
    pub render_layout: RenderLayout,
    pub render_direction: RenderDirection,
    pub sort_property: Option<RasterizationProperty>,
    pub dedup_property: Option<RasterizationProperty>,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            render_height: 8.0,
            render_layout: RenderLayout::Squarish,
            render_direction: RenderDirection::LeftToRight,
            sort_property: Some(RasterizationProperty::Brightness),
            dedup_property: Some(RasterizationProperty::Brightness),
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum RenderLayout {
    Squarish,
    Horizontal,
    Vertical,
}

impl Display for RenderLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Squarish => write!(f, "Squarish"),
            Self::Horizontal => write!(f, "Horizontal"),
            Self::Vertical => write!(f, "Vertical"),
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
}

impl From<RenderData> for ColorImage {
    fn from(value: RenderData) -> Self {
        ColorImage::from_gray([value.width, value.height], &value.pixels)
    }
}


#[derive(Default, Clone)]
pub struct RenderInfo {
    pub cell_size: (usize, usize),
    pub cell_count: (usize, usize)
}


pub fn generate_render_data(
    rasterizations: Rasterizations,
    render_settings: &RenderSettings,
) -> (RenderData,  RenderInfo) {

    let cell_width = rasterizations
        .iter()
        .map(|(m, _)| m.width)
        .max()
        .unwrap_or(render_settings.render_height.ceil() as usize);
    let cell_height = rasterizations
        .iter()
        .map(|(m, _)| m.height)
        .max()
        .unwrap_or(render_settings.render_height.ceil() as usize);



    // Texture dimension in cell counts.
    let (cell_h_count, cell_v_count) = match render_settings.render_layout {
        RenderLayout::Horizontal => (rasterizations.len(), 1),
        RenderLayout::Vertical => (1, rasterizations.len()),
        RenderLayout::Squarish => {
            let total_pixels = cell_width * cell_height * rasterizations.len();
            let target_width = (total_pixels as f32).sqrt().ceil();
            let h_count = (target_width / (cell_width as f32)).round() as usize;
            (h_count, (rasterizations.len() as f32 / h_count as f32).ceil() as usize)
        }
    };

    // Whole texture size in pixels.
    let width = cell_width * cell_h_count;
    let height = cell_height * cell_v_count;

    // Pixel buffer.
    let mut pixels = vec![0u8; width * height];

    for (i, (metrics, rasterization)) in rasterizations.iter().enumerate() {

        // Cell coordinates.
        let (cell_x, cell_y) = match render_settings.render_direction {
            RenderDirection::LeftToRight => {
                let x = i % cell_h_count;
                let y = i / cell_h_count;
                (x, y)
            },
            RenderDirection::TopToBottom => {
                let x = i / cell_v_count;
                let y = i % cell_v_count;
                (x, y)
            }
        };

        // Rasterization offset within cell.
        let center_offset_x = ((cell_width as isize) - (metrics.width as isize)) / 2;
        let center_offset_y = ((cell_height as isize) - (metrics.height as isize)) / 2;

        for (i, value) in rasterization.iter().enumerate() {
            
            // Pixel coordinate within character rasterization.
            let raster_relative_x = i % metrics.width;
            let raster_relative_y = (i - raster_relative_x) / metrics.width;

            // Pixel coordinate within cell.
            let cell_relative_x = raster_relative_x as isize + center_offset_x;
            let cell_relative_y = raster_relative_y as isize + center_offset_y;
            // let center_offset_y = (cell_height - metrics.height) as isize;

            // let inverted_ymin =
            //     cell_height as isize - (metrics.height as isize + metrics.ymin as isize);
            let inverted_ymin = 0;

            // Absolute pixel coordinate within texture atlas.
            let x = ((cell_x * cell_width) as isize 
                + cell_relative_x
                + metrics.xmin as isize)
                .max(0) as usize;
            let y = ((cell_y * cell_height) as isize
                + cell_relative_y
                + inverted_ymin)
                .max(0) as usize;
            // let x = (x as i32 + metrics.xmin) as usize + ((cell_width - metrics.width) / 2);
            // let y = (y as i32 + invert_ymin(metrics.ymin, pixel_height as usize, metrics.height))
            //     as usize
            //     + ((cell_height - metrics.height) / 2);

            // Absolute pixel coordinate as index in pixel buffer.
            let index = x + (y * cell_h_count * cell_width);
            if let Some(pixel) = pixels.get_mut(index) {
                *pixel = *value;
            }
        }
    }

    (
        RenderData {
            width,
            height,
            pixels
        },
        RenderInfo {
            cell_size: (cell_width, cell_width),
            cell_count: (cell_h_count, cell_v_count)
        }
    )
    // (texture_width, texture_height, pixels)
}


// pub fn render_image(
//     mut rasterizations: Rasterizations,
//     render_settings: &RenderSettings
// ) -> Result<ColorImage, RendererError>  {

//     if let Some(p) = render_settings.sort_property {
//         rasterizations.sort_rasters_by(p);
//     }

//     if let Some(p) = render_settings.dedup_property {
//         rasterizations.dedup_rasters_by(p);
//     }

//     let (render_data, _render_info) = generate_render_data(rasterizations, render_settings);

//     if render_data.width.max(render_data.height) <= 16384 {
//         Ok(ColorImage::from_gray([render_data.width, render_data.height], &render_data.pixels))
//     } else {
//         Err(RendererError::TooLarge)
//     }
// }


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
    
    // render_path.push(!("{}.png"));

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
