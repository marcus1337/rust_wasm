
mod app_init;
mod app_gui;
use bevy::prelude::*;
use app_init::initialize_app;
use app_gui::initialize_gui;


pub fn init_and_run(canvas_id: Option<String>) {
    let mut app = App::new();
    initialize_app(&mut app, canvas_id.clone());
    initialize_gui(&mut app);
    app.run();
}

