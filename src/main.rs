// use dialoguer::Select;

use eframe::{run_native, Result, NativeOptions};
use rasterizer::
    app::FontRasterizerApp
;


fn main() -> Result {
    let native_options = NativeOptions::default();
    run_native(
        "Font Rasterizer", 
        native_options, 
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<FontRasterizerApp>::default())
        })
    )
}

// fn main() -> Result<(), AppError> {
//     println!("Please choose a font to rasterize:");

//     let font_face = app::get_font_face()?;

//     println!("{}", font_face.path());

//     println!("Enter font render height (in pixels):");
//     let pixel_height;
//     loop {
//         if let Some(height) = get_input::<f32>().ok() {
//             pixel_height = height;
//             break;
//         } else {
//             println!("Error parsing input. Please enter cell height (in whole pixels):");
//         }
//     }

//     println!(
//         "Generating rasterization data for '{}' at a render height of {} pixel(s)... ",
//         font_face.stem(),
//         pixel_height
//     );

//     let mut rasterizations: Rasterizations = font_face.rasterize(None, pixel_height);

//     println!("Done, generated {} character rasterizations", {
//         rasterizations.len()
//     });

//     let rendering_layouts = vec![
//         RenderingLayout::Squarish,
//         RenderingLayout::Horizontal,
//         RenderingLayout::Vertical,
//     ];

//     // let layout_selection = Select::new()
//     //     .with_prompt("Please choose a rendering layout")
//     //     .items(&rendering_layouts)
//     //     .interact()
//     //     .unwrap();

//     let rendering_layout = rendering_layouts.into_iter().nth(layout_selection).unwrap();

//     let properties = vec![
//         None,
//         Some(RasterizationProperty::Brightness),
//         Some(RasterizationProperty::Width),
//         Some(RasterizationProperty::Height),
//     ];

//     let sort_property_selection = Select::new()
//         .with_prompt("Please select a property to sort by")
//         .items(
//             &properties
//                 .iter()
//                 .map(|o| {
//                     if let Some(p) = o {
//                         p.to_string()
//                     } else {
//                         "None".to_owned()
//                     }
//                 })
//                 .collect::<Vec<String>>(),
//         )
//         .interact()
//         .unwrap();

//     let sort_property = properties.iter().nth(sort_property_selection).unwrap();

//     if let Some(p) = sort_property {
//         rasterizations.sort_rasters_by(p.to_owned());
//     }

//     let property_duplicate_text = &properties
//         .iter()
//         .map(|o| {
//             if let Some(p) = o {
//                 format!(
//                     "{} - {} duplicate(s)",
//                     p,
//                     rasterizations.count_duplicates(p.to_owned())
//                 )
//             } else {
//                 "None".to_owned()
//             }
//         })
//         .collect::<Vec<String>>();

//     let dedup_property_selection = Select::new()
//         .with_prompt("Please select a property to remove duplicates")
//         .items(property_duplicate_text)
//         .interact()
//         .unwrap();

//     let dedup_property = properties
//         .into_iter()
//         .nth(dedup_property_selection)
//         .unwrap();

//     if let Some(p) = dedup_property {
//         rasterizations.dedup_rasters_by(p);
//     }
    


//     let rendering_directions = vec![RenderingDirection::LeftToRight, RenderingDirection::TopToBottom];
//     let rendering_direction_selection = Select::new()
//         .with_prompt("Please select rendering direction")
//         .items(&rendering_directions)
//         .interact()
//         .unwrap();
//     let rendering_direction = rendering_directions.into_iter().nth(rendering_direction_selection).unwrap();


//     let max_width = rasterizations
//         .iter()
//         .map(|(m, _)| m.width)
//         .max()
//         .unwrap_or(pixel_height.ceil() as usize);
//     let max_height = rasterizations
//         .iter()
//         .map(|(m, _)| m.height)
//         .max()
//         .unwrap_or(pixel_height.ceil() as usize);

//     let (pixel_width, pixel_height, pixels) = generate_image_data(
//         max_width,
//         max_height,
//         // pixel_height,
//         rasterizations,
//         rendering_layout,
//         rendering_direction
//     );

//     let image = Image::Grayscale(
//         format!(
//             "{}-(w{}-h{})",
//             font_face.stem().to_owned(),
//             max_width,
//             max_height,
//         ),
//         pixel_width,
//         pixel_height,
//         pixels,
//     );

//     write_image(image).map_err(|err| AppError::RenderingError(err))?;

//     Ok(())
// }
