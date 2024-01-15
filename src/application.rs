use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use adw::{gio, glib};
use adw::glib::Object;
use adw::prelude::*;
use adw::subclass::prelude::*;
use directories::ProjectDirs;
use lazy_static::lazy_static;
use url::Url;
use crate::model::{Trainer, Word};
use crate::view::statistic::StatisticWindow;
use crate::view::window::Window;

const APP_ID: &str = "at.ac.tgm.pdamianik.spelling_trainer";
lazy_static!{
    static ref SAVE_FOLDER: PathBuf = ProjectDirs::from("at.ac", "tgm", "spelling_trainer").expect("Failed to get project dirs").data_dir().to_owned();
    static ref SAVE_FILE: PathBuf = SAVE_FOLDER.join("save.cbor");
    static ref WORDS: [Word; 4] = [
        Word {
            word: "apple".to_owned(),
            url: Url::parse("https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fwikiclipart.com%2Fwp-content%2Fuploads%2F2016%2F09%2Fclip-art-apple-free-clipart.png&f=1&nofb=1&ipt=8d2d625dc550c18588574defee43dedc1906b3ff464fb1afab521c3426ed6f0e&ipo=images").expect("Failed to parse builtin image url"),
            credits: "apple https://wikiclipart.com/apple-clipart_480/".to_owned(),
        },
        Word {
            word: "raspberry".to_owned(),
            url: Url::parse("https://external-content.duckduckgo.com/iu/?u=http%3A%2F%2Fclipartmag.com%2Fimages%2Fraspberry-clipart-27.png&f=1&nofb=1&ipt=e7b96c092eb38787f696e47f6c80cb23b5e7c299a538f01a099597fb06ba0f21&ipo=images").expect("Failed to parse builtin image url"),
            credits: "raspberry https://clipartmag.com/download-clipart-image#raspberry-clipart-27.png".to_owned(),
        },
        Word {
            word: "dog".to_owned(),
            url: Url::parse("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwallpaperboat.com%2Fwp-content%2Fuploads%2F2021%2F05%2F13%2F77274%2Fdoge-meme-11.jpg&f=1&nofb=1&ipt=2c90776ba562173dcbda96b9fa10110e2ec577700a7f8ec511dc9a2825b644b9&ipo=images").expect("Failed to parse builtin image url"),
            credits: "dog https://wallpaperboat.com/doge-meme-wallpapers".to_owned(),
        },
        Word {
            word: "cat".to_owned(),
            url: Url::parse("https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fthecaninebuddy.com%2Fwp-content%2Fuploads%2F2021%2F08%2Fcrying-cat-meme.jpg&f=1&nofb=1&ipt=e2f2214f7587939060fef5208b166c8e19269d2a4b92b6185f8f83119bff266b&ipo=images").expect("Failed to parse builtin image url"),
            credits: "cat https://thecaninebuddy.com/crying-cat-meme-know-when-you-should-use-it/".to_owned(),
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

        dialog.add_credit_section(Some("Images"), &WORDS.iter().map(|word| word.credits.as_str()).collect::<Vec<_>>());

        dialog.present();
    }

    pub fn show_statistic_dialog(&self) {
        let window = self.active_window().unwrap();
        let dialog = StatisticWindow::new(self, &window);

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
            gio::ActionEntry::builder("statistic")
                .activate(move |application: &Application, _, _| {
                    application.show_statistic_dialog();
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
    use super::{SAVE_FILE, SAVE_FOLDER};
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
            let trainer = if SAVE_FILE.exists() {
                ciborium::from_reader(std::fs::File::open(SAVE_FILE.as_path()).expect("Failed to open save file")).expect("Failed to deserialize save file")
            } else {
                let mut trainer = Trainer::new(&super::WORDS[..]);
                trainer.random();
                trainer
            };
            Self {
                trainer: Rc::new(RefCell::new(trainer)),
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

        fn shutdown(&self) {
            self.parent_shutdown();

            if !SAVE_FOLDER.exists() {
                std::fs::create_dir_all(SAVE_FOLDER.as_path()).expect("Failed to create save folder");
            }

            ciborium::into_writer(&*self.trainer.borrow(), std::fs::File::create(SAVE_FILE.as_path()).expect("Failed to create save file")).expect("Failed to serialize save file");
        }
    }
    impl GtkApplicationImpl for Application {}

    impl AdwApplicationImpl for Application {}
}