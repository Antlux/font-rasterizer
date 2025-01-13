use std::
    fmt::Display
;

use eframe::egui::{self, ComboBox, DragValue, Ui};

use crate::{
    rasterization::{FontFace, FontFaceError, RasterizationProperty},
    renderer::{RenderDirection, RenderLayout, RenderSettings, RendererError},
};


#[derive(Default)]
pub struct FontRasterizerApp {
    font_face: Option<FontFace>,
    render_settings: RenderSettings
}


impl FontRasterizerApp {
    fn load_font(&mut self) {
        self.font_face = get_font_face().ok();
    }

    fn center_head(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui|{
            ui.heading("Font Rasterizer");
            ui.separator();
            ui.label("Font:");

            let button_text = if let Some(font_face) = &self.font_face {
                font_face.stem()
            } else {
                "Load"
            };

            if ui.button(button_text).clicked() {
                self.load_font();
            }
        });
    }

    fn settings_body(&mut self, ui: &mut Ui) {
        // Render Height
        ui.add(DragValue::new(&mut self.render_settings.render_height).range(1..=100).speed(0.1));

        // Render Layout
        ComboBox::from_label("Render Layout")
            .selected_text(format!("{}", self.render_settings.render_layout.to_string()))
            .show_ui(ui, |ui| {
                let layouts = vec![RenderLayout::Squarish, RenderLayout::Horizontal, RenderLayout::Vertical];
                for l in layouts {
                    ui.selectable_value(
                        &mut self.render_settings.render_layout,
                        l,
                        l.to_string()
                    );
                }
            });
        
        // Render Direction
        ComboBox::from_label("Render Direction")
            .selected_text(format!("{}", self.render_settings.render_direction.to_string()))
            .show_ui(ui, |ui| {
                let directions = vec![RenderDirection::LeftToRight, RenderDirection::TopToBottom];
                for d in directions {
                    ui.selectable_value(
                        &mut self.render_settings.render_direction,
                        d,
                        d.to_string()
                    );
                }
            });
        
        // Sort Property
        ComboBox::from_label("Sort Property")
            .selected_text(
                format!("{}", {
                    if let Some(p) = self.render_settings.sort_property {
                        p.to_string()
                    } else {
                        "None".into()
                    }
                })
            )
            .show_ui(ui, |ui| {
                let properties = vec![None, Some(RasterizationProperty::Brightness), Some(RasterizationProperty::Width), Some(RasterizationProperty::Height)];
                for p in properties {
                    ui.selectable_value(
                        &mut self.render_settings.sort_property,
                        p,
                        format!("{}", {
                            if let Some(p) = p {
                                p.to_string()
                            } else {
                                "None".into()
                            }
                        })
                    );
                }
            });
        
        // Dedup Property
        ComboBox::from_label("Dedup Property")
            .selected_text(
                format!("{}", {
                    if let Some(p) = self.render_settings.dedup_property {
                        p.to_string()
                    } else {
                        "None".into()
                    }
                })
            )
            .show_ui(ui, |ui| {
                let properties = vec![None, Some(RasterizationProperty::Brightness), Some(RasterizationProperty::Width), Some(RasterizationProperty::Height)];
                for p in properties {
                    ui.selectable_value(
                        &mut self.render_settings.dedup_property,
                        p,
                        format!("{}", {
                            if let Some(p) = p {
                                p.to_string()
                            } else {
                                "None".into()
                            }
                        })
                    );
                }
            });
    }


}

impl eframe::App for FontRasterizerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.center_head(ui);
            ui.separator();
            
        });

        if let Some(_font_face) = &self.font_face {
            egui::SidePanel::right("side-panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.heading("Render Settings");
                ui.separator();
                self.settings_body(ui);
                ui.separator()
                // Export History
            });
        }
    }
}








#[derive(Debug)]
pub enum AppError {
    NoFontPath,
    InvalidFontPath,
    FontLoadingError(FontFaceError),
    MissingCellDim,
    InputParsingError,
    RenderingError(RendererError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFontPath => write!(f, "No path was provided."),
            Self::InvalidFontPath => write!(f, "Invalid Path."),
            Self::FontLoadingError(err) => write!(f, "Encountered error loading font: {err}."),
            Self::MissingCellDim => write!(f, "Must provide dimension."),
            Self::InputParsingError => write!(f, "Encountered error parsing user input."),
            Self::RenderingError(err) => write!(f, "Encountered error rendering: {err}."),
        }
    }
}

pub fn get_font_face() -> Result<FontFace, AppError> {
    let font_path = rfd::FileDialog::new()
        .add_filter("font", &["ttf", "ttc", "otf"])
        .set_directory("/")
        .pick_file()
        .ok_or(AppError::NoFontPath)?;
    FontFace::load(font_path).map_err(|err| AppError::FontLoadingError(err))
}

// pub fn get_input<T>() -> Result<T, <T as FromStr>::Err>
// where
//     T: ToString + FromStr,
//     <T as FromStr>::Err: Debug,
// {
//     let mut input_buf = String::new();
//     let _ = stdin().read_line(&mut input_buf);
//     input_buf.trim().split(' ').next().unwrap().parse::<T>()
// }
