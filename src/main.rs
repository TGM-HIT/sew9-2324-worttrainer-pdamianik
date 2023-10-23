mod model;
mod view;

use adw::Application;
use gtk::prelude::*;
use gtk::glib;
use crate::view::build_ui;

const APP_ID: &str = "at.ac.tgm.pdamianik.spelling_trainer";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}