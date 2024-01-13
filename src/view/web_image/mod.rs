use gtk::glib;
use gtk::glib::Object;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use reqwest::IntoUrl;

mod util;

glib::wrapper! {
    pub struct WebImage(ObjectSubclass<imp::WebImage>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for WebImage {
    fn default() -> Self {
        Object::new()
    }
}

impl WebImage {
    pub fn new() -> Self {
        Self::default()
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

mod imp {
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
}
