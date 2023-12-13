use std::cell::RefCell;
use gtk::glib;
use gtk::glib::subclass::prelude::*;
use gtk::{Image, Spinner};
use gtk::prelude::*;
use gtk::subclass::prelude::*;

#[derive(Default)]
pub struct WebImage {
    pub image: RefCell<Image>,
    pub spinner: RefCell<Spinner>,
}

#[glib::object_subclass]
impl ObjectSubclass for WebImage {
    const NAME: &'static str = "WebImage";

    type Type = super::WebImage;

    type ParentType = gtk::Box;
}

impl ObjectImpl for WebImage {
    fn constructed(&self) {
        let obj = self.obj();
        self.parent_constructed();
        let image = Image::builder()
            .vexpand(true)
            .hexpand(true)
            .visible(false)
            .build();
        self.image.replace(image.clone());
        obj.append(&image);
        let spinner = Spinner::builder()
            .width_request(30)
            .height_request(30)
            .spinning(false)
            .visible(false)
            .build();
        self.spinner.replace(spinner.clone());
        obj.append(&spinner);
    }
}

impl WidgetImpl for WebImage {}

impl BoxImpl for WebImage {}