use adw::Application;
use gtk::prelude::*;

mod web_image;
mod main_window;
mod word_list;

use main_window::Window;

pub fn build_ui(app: &Application) {
    let window = Window::new(app);
    window.present();
}
