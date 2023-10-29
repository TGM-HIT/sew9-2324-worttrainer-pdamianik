use adw::gio;
use adw::glib::ExitCode;
use adw::prelude::*;
use crate::application::Application;

mod model;
mod view;
mod application;

fn main() -> ExitCode {
    gio::resources_register_include!("spelling_trainer.gresource")
        .expect("failed to register resources.");

    Application::new().run()
}
