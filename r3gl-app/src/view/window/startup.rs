use egui::{RichText, Link, TextStyle, ScrollArea, Label, Sense, Align2, vec2, FontId, Color32};
use wcore::{graphics::context::Context, egui::window::Window};
use str_macro::str;

use crate::state::State;

pub struct StartupWindow {
    visible: bool
}

impl StartupWindow {
    pub fn new() -> Self {
        return Self {
            visible: true
        };
    }
}

impl Window<&mut State> for StartupWindow {
    type Title = &'static str;
    fn title() -> Self::Title {
        return "Startup";
    }

    #[allow(unused_variables)]
    fn build<'a>(window: egui::Window<'a>, ctx: &'_ egui::Context) -> egui::Window<'a> {
        window
            .anchor(Align2::CENTER_TOP, vec2(0.0, 96.0))
            .fixed_size(vec2(360.0, 400.0))
            .min_height(600.0)
            .collapsible(false)
            .title_bar(false)
    }

    fn set_visible(&mut self, value: bool) { self.visible = value; }
    fn get_visible(&self) -> bool { return self.visible; }

    #[allow(unused_variables)]
    #[allow(unused_must_use)] // Until I figure out a better way to precalculate width
    fn show(&mut self, state: &mut State, view: &wgpu::TextureView, graphics: &mut Context, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.group(|ui| {
                ui.label(RichText::new("r3gl-client").heading().strong());
                ui.label("experimental beatmap editor");
            });
        });

        ui.add_space(4.0);

        ui.horizontal(|ui| {
            // Setup layout
            ui.spacing_mut().item_spacing.x = 0.0;

            let width = ui.available_width();
            let text_width = ui.painter().layout(
                str!("open folder | open file"),
                FontId::default(),
                Color32::default(),
                width
            ).rect.width();

            // TODO: might wanna cache this all, eh?
            let offset = (width - text_width) / 2.0;
            ui.add_space(offset);

            // Menu    
            if ui.add(Link::new("open folder")).clicked() {
                // self.filepicker.open();
            };

            ui.label(" | ");
            
            if ui.add(Link::new("open file")).clicked() {
                // self.filepicker.open();
            }
        });

        ui.add_space(4.0);
        
        ui.with_layout(ui.layout().with_cross_justify(true), |ui| {
            ui.group(|ui| {
                ui.label(RichText::new("Projects").heading().strong());
                ui.separator();

                let text_style = TextStyle::Body;
                let row_height = ui.text_style_height(&text_style);
                let num_rows = 10;
                ScrollArea::vertical()
                  .auto_shrink([true, false])
                  .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    let mut recent = vec![]; // I hate rust
                    for project in state.projects.recent.iter().skip(row_range.start).take(row_range.end) {
                        let text = format!("• {}", project.name);
                        let label = Label::new(text).wrap(false).sense(Sense::click());
                        recent.push((ui.add(label), project.path.clone()));
                    }

                    for (button, path) in recent {
                        if button.clicked() {
                            state.editor.open_project(&path, &mut state.projects);
                        }
                    }
                });

            });
        });
    }
}