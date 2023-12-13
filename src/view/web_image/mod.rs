use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use reqwest::IntoUrl;

mod util;
mod imp;

glib::wrapper! {
    pub struct WebImage(ObjectSubclass<imp::WebImage>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl WebImage {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub async fn load(&self, url: impl IntoUrl + 'static) -> Result<(), util::Error> {
        let image = self.imp().image.clone();
        let spinner = self.imp().spinner.clone();
        image.borrow().set_visible(false);
        spinner.borrow().set_visible(true);
        spinner.borrow().start();
        let image_data = util::load_image(url).await
            .expect("Failed loading image data");
        image.borrow().set_from_paintable(image_data.as_ref());
        spinner.borrow().stop();
        spinner.borrow().set_visible(false);
        image.borrow().set_visible(true);
        Ok(())
    }
}