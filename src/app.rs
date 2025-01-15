use std::
    fmt::Display
;

use eframe::egui::{self, load::SizedTexture, ColorImage, ComboBox, DragValue, Image, ImageData, TextureOptions, Ui};

use crate::{
    font_face::{FontFace, FontFaceError}, rasterization::{RasterManip, RasterizationProperty}, renderer::{generate_render_data, write_image, RenderData, RenderDirection, RenderInfo, RenderLayout, RenderSettings, RendererError}
};


#[derive(Default)]
pub struct FontRasterizerApp {
    font_face: Option<FontFace>,
    render_settings: RenderSettings,
    render_data: RenderData,
    render_info: RenderInfo,
    render: Option<ColorImage>,
}


impl FontRasterizerApp {
    fn load_font(&mut self) {
        if let Ok(font_face) = get_font_face() {
            self.font_face = Some(font_face);
        }
        self.render_font();
    }

    fn render_font(&mut self) {
        if let Some(font_face) = &self.font_face {
            
            let mut rasterizations = font_face.rasterize(None, self.render_settings.render_height);

            if let Some(p) = self.render_settings.dedup_property {
                rasterizations.dedup_rasters_by(p);
            } else if self.render_settings.dedup_exact_duplicate {
                rasterizations.dedup_exact_duplicate();
            }

            if let Some(p) = self.render_settings.sort_property {
                rasterizations.sort_rasters_by(p);
            } 

            let (render_data, render_info) = generate_render_data(rasterizations, &self.render_settings);

            if render_data.renderable() {
                self.render_data = render_data.clone();
                self.render_info = render_info;
                self.render = Some(ColorImage::from(render_data));
            }
        }
    }

    fn export_texture(&mut self) {
        if let Some(font_face) = &self.font_face {
            let (cell_width, cell_height) = self.render_info.cell_size;
            let (cell_h_count, cell_v_count) = self.render_info.cell_count;
            let texture_name = format!("{}-({}w-{}h)-({}H-{}V)", font_face.stem(), cell_width, cell_height, cell_h_count, cell_v_count);
            if let Err(err) = write_image(texture_name, &self.render_data) {
                eprintln!("{}", err);
            }
        }
    }

    fn header(&mut self, ui: &mut Ui) {
        ui.vertical(|ui|{
            ui.heading("Font Rasterizer");

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Font:");
                if ui.button(
                    if let Some(font_face) = &self.font_face {
                        font_face.stem()
                    } else {
                        "Load"
                    }
                ).clicked() {
                    self.load_font();
                }
                if self.render.is_some() {
                    ui.separator();
    
                    if ui.button("Export Texture").clicked() {
                        self.export_texture();
                    }
                }
            });

        });
    }

    fn settings_body(&mut self, ui: &mut Ui) {

        ui.horizontal(|ui| {
            ui.label("Render Height");
            let resp = ui.add(DragValue::new(&mut self.render_settings.render_height).range(1..=100).speed(0.1));
            if resp.drag_stopped() || resp.lost_focus() {
                self.render_font();
            }
        });
        // Render Height

        // Render Layout
        ComboBox::from_label("Render Layout")
            .selected_text(format!("{}", self.render_settings.render_layout.to_string()))
            .show_ui(ui, |ui| {
                let layouts = if let RenderLayout::Custom(h, v) = self.render_settings.render_layout {
                    vec![RenderLayout::Squarish, RenderLayout::Horizontal, RenderLayout::Vertical, RenderLayout::Packed(false), RenderLayout::Custom(h, v)]
                } else if let RenderLayout::Packed(flipped) = self.render_settings.render_layout {
                    vec![RenderLayout::Squarish, RenderLayout::Horizontal, RenderLayout::Vertical, RenderLayout::Packed(flipped), RenderLayout::Custom(10, 10)]
                } else {
                    vec![RenderLayout::Squarish, RenderLayout::Horizontal, RenderLayout::Vertical, RenderLayout::Packed(false), RenderLayout::Custom(10, 10)]
                };

                for l in layouts {
                    if ui.selectable_value(
                        &mut self.render_settings.render_layout,
                        l,
                        l.to_string()
                    ).changed() {
                        self.render_font();
                    };
                }
            });
        
        if let RenderLayout::Custom(mut h, mut v) = self.render_settings.render_layout {
            let mut h_b = false;
            let mut v_b = false;
            ui.horizontal(|ui| {
                ui.label("Width");
                let resp = ui.add(DragValue::new(&mut h).range(1..=1000).speed(1.0));
                h_b = resp.drag_stopped() || resp.lost_focus();
                
            });
            ui.horizontal(|ui| {
                ui.label("Height");
                let resp = ui.add(DragValue::new(&mut v).range(1..=1000).speed(1.0));
                v_b = resp.drag_stopped() || resp.lost_focus();
            });

            self.render_settings.render_layout = RenderLayout::Custom(h, v);

            if h_b || v_b {
                self.render_font();
            }
        } 
        if let RenderLayout::Packed(mut flipped) = self.render_settings.render_layout {
            if ui.checkbox(&mut flipped, "Flipped").changed() {
                self.render_settings.render_layout = RenderLayout::Packed(flipped);
                self.render_font();
            }
        }

        // Render Direction
        ComboBox::from_label("Render Direction")
            .selected_text(format!("{}", self.render_settings.render_direction.to_string()))
            .show_ui(ui, |ui| {
                let directions = vec![RenderDirection::LeftToRight, RenderDirection::TopToBottom];
                for d in directions {
                    if ui.selectable_value(
                        &mut self.render_settings.render_direction,
                        d,
                        d.to_string()
                    ).changed() {
                        self.render_font();
                    };
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
                    if ui.selectable_value(
                        &mut self.render_settings.sort_property,
                        p,
                        format!("{}", {
                            if let Some(p) = p {
                                p.to_string()
                            } else {
                                "None".into()
                            }
                        })
                    ).changed() {
                        self.render_font();
                    };
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
                    if ui.selectable_value(
                        &mut self.render_settings.dedup_property,
                        p,
                        format!("{}", {
                            if let Some(p) = p {
                                p.to_string()
                            } else {
                                "None".into()
                            }
                        })
                    ).changed() {
                        self.render_font();
                    };
                }
            });
        
        if self.render_settings.dedup_property.is_none() {
            if ui.checkbox(&mut self.render_settings.dedup_exact_duplicate, "Remove only exact duplicates").changed() {self.render_font();};
        }
    }

}

impl eframe::App for FontRasterizerApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        
        egui::SidePanel::left("side-panel")
        .resizable(false)
        .show(ctx, |ui| {
            self.header(ui);
            ui.separator();
            if self.font_face.is_some() {
                ui.heading("Render Settings");
                ui.separator();
                self.settings_body(ui);
            }

        });

        egui::TopBottomPanel::bottom("bottom-panel").show(ctx, |ui| {
            ui.hyperlink_to("Font Rasterizer on Github", "https://github.com/Antlux/font-rasterizer.git")
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let (cell_width, cell_height) = self.render_info.cell_size;
                let (cell_h_count, cell_v_count) = self.render_info.cell_count;
                let texture_width = cell_width * cell_h_count;
                let texture_height = cell_height * cell_v_count;
                let cell_filled = self.render_info.cell_filled;
                let cell_count = cell_h_count * cell_v_count;
                let empty_cells = cell_count - cell_filled;
                let info_text = format!(
                    "{} characters rendered | Cell size: {}x{} pixels | Cell count: {}x{} ({}) | Empty cells: {} | Texture size: {}x{} pixels", 
                    cell_filled, 
                    cell_width, 
                    cell_height, 
                    cell_h_count, 
                    cell_v_count, 
                    cell_count,
                    empty_cells,
                    texture_width,
                    texture_height
                );
                ui.label(info_text)
            });

            ui.separator();

            if let Some(img) = &self.render {
                let render_img = ui.ctx().load_texture(
                    "render", 
                    ImageData::from(img.to_owned()), 
                    TextureOptions::NEAREST
                );
                
                ui.centered_and_justified(|ui| {
                    ui.add(
                        Image::from_texture(SizedTexture::from_handle(&render_img))
                        .fit_to_exact_size(ui.available_size())
                    );
                });
            }
        });        
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
