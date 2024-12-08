use std::{fmt::Display, fs::File, io::BufWriter};

use crate::rasterization::Rasterizations;

pub enum Image {
    Grayscale(String, usize, usize, Vec<u8>),
}

#[derive(Debug)]
pub enum RendererError {
    InvalidPath,
    CreationError,
}

impl Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Path provided is invalid"),
            Self::CreationError => write!(f, "Encountered error creating render file"),
        }
    }
}

pub enum RenderingLayout {
    Squarish,
    Horizontal,
    Vertical,
}

impl Display for RenderingLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Squarish => write!(f, "Squarish"),
            Self::Horizontal => write!(f, "Horizontal"),
            Self::Vertical => write!(f, "Vertical"),
        }
    }
}

// fn invert_ymin(ymin: i32, pixel_height: usize, height: usize) -> i32 {
//     pixel_height as i32 - ymin - height as i32
// }

pub fn generate_image_data(
    cell_width: usize,
    cell_height: usize,
    // pixel_height: f32,
    rasterizations: Rasterizations,
    rendering_layout: RenderingLayout,
) -> (usize, usize, Vec<u8>) {
    let (cell_h_count, cell_v_count) = match rendering_layout {
        RenderingLayout::Horizontal => (rasterizations.len(), 1),
        RenderingLayout::Vertical => (1, rasterizations.len()),
        RenderingLayout::Squarish => {
            let total_pixels = cell_width * cell_height * rasterizations.len();
            let target_width = (total_pixels as f32).sqrt().ceil();
            let h_count = (target_width / (cell_width as f32)).round() as usize;
            (h_count, rasterizations.len() / h_count)
        }
    };

    let texture_width = cell_width * cell_h_count;
    let texture_height = cell_height * cell_v_count;
    let mut pixels = vec![0u8; texture_width * texture_height];

    for (i, (metrics, rasterization)) in rasterizations.iter().enumerate() {
        let cell_x = i % cell_h_count;
        let cell_y = (i - cell_x) / cell_h_count;

        for (i, value) in rasterization.iter().enumerate() {
            let cell_relative_x = i % metrics.width;
            let cell_relative_y = (i - cell_relative_x) / metrics.width;

            let center_offset_x = ((cell_width as isize) - (metrics.width as isize)) / 2;
            let center_offset_y = ((cell_height as isize) - (metrics.height as isize)) / 2;
            // let center_offset_y = (cell_height - metrics.height) as isize;

            // let inverted_ymin =
            //     cell_height as isize - (metrics.height as isize + metrics.ymin as isize);
            let inverted_ymin = 0;

            let x = (((cell_x * cell_width) + cell_relative_x) as isize
                + center_offset_x
                + metrics.xmin as isize)
                .max(0) as usize;
            let y = (((cell_y * cell_height) + cell_relative_y) as isize
                + center_offset_y
                + inverted_ymin)
                .max(0) as usize;
            // let x = (x as i32 + metrics.xmin) as usize + ((cell_width - metrics.width) / 2);
            // let y = (y as i32 + invert_ymin(metrics.ymin, pixel_height as usize, metrics.height))
            //     as usize
            //     + ((cell_height - metrics.height) / 2);
            let index = x + (y * cell_h_count * cell_width);
            if let Some(pixel) = pixels.get_mut(index) {
                *pixel = *value;
            }
        }
    }

    (texture_width, texture_height, pixels)
}

pub fn write_image(image: Image) -> Result<(), RendererError> {
    let Image::Grayscale(name, width, height, data) = image;

    let mut render_path = rfd::FileDialog::new()
        .set_directory("/")
        .add_filter("png", &["png"])
        .pick_folder()
        .ok_or(RendererError::InvalidPath)?;

    render_path.push(format!("{}.png", name));

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
    writer.write_image_data(&data).unwrap();

    Ok(())
}
