use adw::{Application, ApplicationWindow, HeaderBar};
use gtk::{Button, Orientation, Box};
use gtk::prelude::*;

pub fn build_ui(app: &Application) {
    let header = HeaderBar::builder()
        .build();

    let button = Button::builder()
        .label("Hello, World!")
        .margin_top(12)
        .margin_end(12)
        .margin_bottom(12)
        .margin_start(12)
        .build();

    let vbox = Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    vbox.append(&header);
    vbox.append(&button);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Spelling Trainer")
        .content(&vbox)
        .build();
    window.present();
}
