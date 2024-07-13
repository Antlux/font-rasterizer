use std::{fmt::Display, fs::File, io::BufWriter};

pub enum Image {
    Grayscale(String, usize, usize, Vec<u8>),
}


pub enum RendererError {
    InvalidPath,
    CreationError,
}

impl Display for RendererError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "Path provided is invalid"),
            Self::CreationError => write!(f, "Encountered error creating render file")
        }
    }
}


pub fn render(image: Image) -> Result<(), RendererError> {

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
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));     // 1.0 / 2.2, unscaled, but rounded
    let source_chromaticities = png::SourceChromaticities::new(     // Using unscaled instantiation here
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&data).unwrap();

    Ok(())
}