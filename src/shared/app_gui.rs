use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub fn initialize_gui(app: &mut App) {
    app.add_plugins(EguiPlugin);
    app.add_systems(Update, ui_main_menu_system);
}

fn ui_main_menu_system(mut egui_contexts: EguiContexts) {

    let window_color = egui::Color32::from_rgba_premultiplied(128, 128, 128, 200);
    
    egui::Window::new("Main Menu")
    .title_bar(false)
    .resizable(true)
    .collapsible(false)
    .frame(egui::Frame{fill: window_color, ..Default::default()})
    .show(egui_contexts.ctx_mut(), |ui| {
        ui.set_width(500.0);
        ui.set_height(300.0);
        ui.label(egui::RichText::new("Menu!").heading().color(egui::Color32::from_rgb(255, 255, 0)));
        ui.add_space(20.0);
        if ui.button("Button").clicked() {
            println!("Button clicked");
        }
    });
}
