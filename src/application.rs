use adw::{gio, glib};
use adw::glib::Object;
use adw::prelude::*;
use crate::view::window::Window;

const APP_ID: &str = "at.ac.tgm.pdamianik.spelling_trainer";

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl Default for Application {
    fn default() -> Self {
        Object::builder::<Application>()
            .property("application-id", &APP_ID)
            .property("resource-base-path", &"/at/ac/tgm/pdamianik/spelling_trainer")
            .build()
    }
}

impl Application {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn present_window(&self) {
        let window = if let Some(window) = self.active_window() {
            window
        } else {
            let window = Window::new(self);
            window.upcast()
        };
        window.present();
    }
}

mod imp {
    use adw::glib;
    use adw::subclass::prelude::*;

    pub struct Application {

    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "SpellingTrainerApplication";
        type Type = super::Application;
        type ParentType = adw::Application;

        fn new() -> Self {
            Self {}
        }
    }

    impl ObjectImpl for Application {}

    impl ApplicationImpl for Application {
        fn activate(&self) {
            self.parent_activate();

            self.obj().present_window();
        }
    }
    impl GtkApplicationImpl for Application {}

    impl AdwApplicationImpl for Application {}
}