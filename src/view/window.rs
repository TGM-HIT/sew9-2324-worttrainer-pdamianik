use std::cell::RefCell;
use std::rc::Rc;
use glib::Object;
use adw::{gio, glib};
use adw::glib::{clone, IsA, MainContext};
use adw::subclass::prelude::*;
use gtk::prelude::*;
use crate::model::Trainer;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &impl IsA<gtk::Application>) -> Self {
        let window = Object::builder::<Window>()
            .property("application", app)
            .build();
        window.action_set_enabled("win.check", false);
        window.trainer().expect("The application does not have a trainer").borrow_mut().random();
        window.load_image();
        window.imp().image_view.set_center_widget(Some(&window.imp().web_image));
        window
    }

    pub fn trainer(&self) -> Option<Rc<RefCell<Trainer>>> {
        self.application()
            .map(|app| app.downcast::<crate::application::Application>().unwrap().trainer())
    }

    fn load_image(&self) {
        let trainer = self.trainer().expect("The application does not have a trainer");
        let word = trainer.borrow_mut().selected().cloned();

        if let Some(word) = word {
            let main_context = MainContext::default();
            let image = self.imp().web_image.clone();
            main_context.spawn_local(clone!(@strong self as this => async move {
                this.action_set_enabled("win.check", false);
                image.load(word.url.clone()).await
                    .expect("Failed loading image");
                this.action_set_enabled("win.check", true);
            }));
        } else {
            self.action_set_enabled("win.check", false);
        }
    }
}

mod imp {
    use adw::glib::{self};
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{Button, CenterBox, CompositeTemplate, Entry};
    use gtk::prelude::*;
    use crate::view::web_image::WebImage;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/window.ui")]
    pub struct Window {
        #[template_child]
        pub image_view: TemplateChild<CenterBox>,
        #[template_child]
        pub guess_entry: TemplateChild<Entry>,
        #[template_child]
        pub check_button: TemplateChild<Button>,
        pub web_image: WebImage,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "SpellingTrainerWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.install_action("win.check", None, |window, _, _| {
                let entry = window.imp().guess_entry.get();
                let text = entry.buffer().text();
                let check_button = window.imp().check_button.get();
                let trainer = window.trainer().expect("The application does not have a trainer");
                let correct = trainer.borrow_mut().guess(&text);
                if correct {
                    check_button.set_label("Check");
                    entry.buffer().set_text("");
                    trainer.borrow_mut().random();
                    window.load_image();
                } else {
                    check_button.set_label("Try again");
                }
            });

            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
    }

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}
