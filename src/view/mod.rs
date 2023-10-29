use adw::Application;
use gtk::prelude::*;

mod web_image;
mod window;

use window::Window;

pub fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
