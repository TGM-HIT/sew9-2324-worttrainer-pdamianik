use std::cell::RefCell;
use std::rc::Rc;
use adw::{gio, glib};
use adw::glib::Object;
use adw::prelude::*;
use adw::subclass::prelude::*;
use lazy_static::lazy_static;
use url::Url;
use crate::model::{Trainer, Word};
use crate::view::window::Window;

const APP_ID: &str = "at.ac.tgm.pdamianik.spelling_trainer";
lazy_static!{
    static ref WORDS: [Word; 2] = [
        Word {
            word: "apple".to_owned(),
            url: Url::parse("https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fwikiclipart.com%2Fwp-content%2Fuploads%2F2016%2F09%2Fclip-art-apple-free-clipart.png&f=1&nofb=1&ipt=8d2d625dc550c18588574defee43dedc1906b3ff464fb1afab521c3426ed6f0e&ipo=images").expect("Failed to parse builtin image url"),
        },
        Word {
            word: "raspberry".to_owned(),
            url: Url::parse("https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fclipartmag.com%2Fimages%2Fraspberry-clipart-27.png&f=1&nofb=1&ipt=e7b96c092eb38787f696e47f6c80cb23b5e7c299a538f01a099597fb06ba0f21&ipo=images").expect("Failed to parse builtin image url"),
        }
    ];
}

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

    pub fn show_about_dialog(&self) {
        let window = self.active_window().unwrap();
        let dialog = adw::AboutWindow::builder()
            .transient_for(&window)
            .application_name("Spelling Trainer")
            .developer_name("Philip Damianik")
            .version("0.1.0")
            .developers(vec!["Philip Damianik"])
            .build();

        dialog.present();
    }

    pub fn trainer(&self) -> Rc<RefCell<Trainer>> {
        self.imp().trainer.clone()
    }

    fn setup_gactions(&self) {
        self.add_action_entries([
            gio::ActionEntry::builder("about")
                .activate(move |application: &Application, _, _| {
                    application.show_about_dialog();
                })
                .build(),
        ]);
    }
}

mod imp {
    use std::cell::RefCell;
    use std::rc::Rc;
    use adw::glib;
    use adw::subclass::prelude::*;
    use crate::model::Trainer;

    pub struct Application {
        pub trainer: Rc<RefCell<Trainer>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "SpellingTrainerApplication";
        type Type = super::Application;
        type ParentType = adw::Application;

        fn new() -> Self {
            let words = &super::WORDS[..];
            Self {
                trainer: Rc::new(RefCell::new(Trainer::new(words))),
            }
        }
    }

    impl ObjectImpl for Application {
        fn constructed(&self) {
            self.obj().setup_gactions();
        }
    }

    impl ApplicationImpl for Application {
        fn activate(&self) {
            self.parent_activate();

            self.obj().present_window();
        }
    }
    impl GtkApplicationImpl for Application {}

    impl AdwApplicationImpl for Application {}
}