use egui::RichText;
use wcore::{graphics::context::Context, egui::window::Window, bindings::BindingManager};

use crate::{state::State, identifier::Identifier};

pub struct BindingsWindow {
    visible: bool
}

impl BindingsWindow {
    pub fn new() -> Self {
        return Self {
            visible: false
        };
    }
}

impl Window<(&mut State, &mut BindingManager<State, Identifier>)> for BindingsWindow {
    type Title = &'static str;
    fn title() -> Self::Title {
        return "Bindings";
    }

    #[allow(unused_variables)]
    fn build<'a>(window: egui::Window<'a>, ctx: &'_ egui::Context) -> egui::Window<'a> {
        window
            .default_pos([64.0, 64.0])
            .default_size([220.0, 260.0])
            .collapsible(true)
            .resizable(true)
            .title_bar(true)
    }

    fn set_visible(&mut self, value: bool) { self.visible = value; }
    fn get_visible(&self) -> bool { return self.visible; }

    #[allow(unused_variables)]
    fn show(&mut self, (state, binding_manager): (&mut State, &mut BindingManager<State, Identifier>), view: &wgpu::TextureView, graphics: &mut Context, ui: &mut egui::Ui) {
        for (category, bindings) in binding_manager.iter() {
            ui.label(RichText::new(category.to_string()).heading().strong());
            for (bind, action) in bindings {
                egui::Grid::new("bindings_grid")
                  .num_columns(2)
                  .spacing([40.0, 4.0])
                  .striped(true)
                  .show(ui, |ui| {
                    ui.label(&action.name);
                    ui.vertical_centered_justified(|ui| {
                        if ui.button(&format!("{}", bind)).clicked() {
                            
                        }
                    });

                    ui.end_row();
                });
            }
        }

        ui.allocate_space(ui.available_size());
    }
}