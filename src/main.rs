use adw::glib::ExitCode;
use adw::prelude::*;
use adw::{gio, Application};

mod model;
mod observer;
mod view;

use view::build_ui;

const APP_ID: &str = "at.ac.tgm.pdamianik.spelling_trainer";

#[tokio::main]
async fn main() -> ExitCode {
    gio::resources_register_include!("spelling_trainer.gresource")
        .expect("failed to register resources.");

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
