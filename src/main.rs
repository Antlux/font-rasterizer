use eframe::{run_native, Result, NativeOptions};
use font_rasterizer::app::FontRasterizerApp;

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

