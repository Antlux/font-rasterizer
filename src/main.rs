use dialoguer::Select;

use rasterizer::{
    app::{self, get_input, AppError},
    rasterization::Rasterizations,
    renderer::{generate_image_data, write_image, Image, RenderingLayout},
};

fn main() -> Result<(), AppError> {
    println!("Please choose a font to rasterize");

    let font_face = app::get_font_face()?;

    // println!("Enter cell width (in whole pixels):");
    // let cell_width;
    // loop {
    //     if let Some(width) = get_input::<usize>().ok() {
    //         cell_width = width;
    //         break;
    //     } else {
    //         println!("Error parsing input. Please enter cell height (in whole pixels):");
    //     }
    // }

    // println!("Enter cell height (in whole pixels):");
    // let cell_height;
    // loop {
    //     if let Some(height) = get_input::<usize>().ok() {
    //         cell_height = height;
    //         break;
    //     } else {
    //         println!("Error parsing input. Please enter cell height (in whole pixels):");
    //     }
    // }

    println!("Enter font render height (in pixels) :");
    let pixel_height;
    loop {
        if let Some(height) = get_input::<f32>().ok() {
            pixel_height = height;
            break;
        } else {
            println!("Error parsing input. Please enter cell height (in whole pixels):");
        }
    }

    println!(
        "Generating rasterization data for '{}' at a render height of {} pixel(s)... ",
        font_face.stem(),
        pixel_height
    );

    let rasterizations: Rasterizations = font_face.rasterize(None, pixel_height);

    println!("Done, generated {} character rasterizations", {
        rasterizations.len()
    });

    let rendering_layouts = vec![
        RenderingLayout::Squarish,
        RenderingLayout::Horizontal,
        RenderingLayout::Vertical,
    ];

    let layout_selection = Select::new()
        .with_prompt("Please choose a rendering layout")
        .items(&rendering_layouts)
        .interact()
        .unwrap();

    let rendering_layout = rendering_layouts.into_iter().nth(layout_selection).unwrap();

    let max_width = rasterizations
        .iter()
        .map(|(m, _)| m.width)
        .max()
        .unwrap_or(pixel_height.ceil() as usize);
    let max_height = rasterizations
        .iter()
        .map(|(m, _)| m.height)
        .max()
        .unwrap_or(pixel_height.ceil() as usize);

    let (pixel_width, pixel_height, pixels) = generate_image_data(
        max_width,
        max_height,
        pixel_height,
        rasterizations,
        rendering_layout,
    );

    let image = Image::Grayscale(
        format!(
            "{}-(w{}-h{})",
            font_face.stem().to_owned(),
            max_width,
            max_height,
        ),
        pixel_width,
        pixel_height,
        pixels,
    );

    write_image(image).map_err(|err| AppError::RenderingError(err))?;

    Ok(())
}
