use glib::Object;
use adw::{gio, glib};
use adw::glib::IsA;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &impl IsA<gtk::Application>) -> Self {
        Object::builder().property("application", app).build()
    }
}

mod imp {
    use adw::gio::SimpleAction;
    use crate::view::web_image::load_image;
    use adw::glib::{self, clone, MainContext};
    use adw::subclass::prelude::*;
    use glib::subclass::InitializingObject;
    use gtk::{Button, CompositeTemplate, Entry, Image, Spinner};
    use gtk::prelude::*;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/window.ui")]
    pub struct Window {
        #[template_child]
        pub image: TemplateChild<Image>,
        #[template_child]
        pub spinner: TemplateChild<Spinner>,
        #[template_child]
        pub guess_entry: TemplateChild<Entry>,
        #[template_child]
        pub check_button: TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "SpellingTrainerWindow";
        type Type = super::Window;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for Window {
        fn constructed(&self) {
            self.parent_constructed();

            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@strong self.image as image, @strong self.spinner as spinner => async move {
                let image_data = load_image("https://source.unsplash.com/random").await
                    .expect("Failed loading image data");
                image.set_from_paintable(image_data.as_ref());
                spinner.set_visible(false);
                image.set_visible(true);
            }));

            let action_check = SimpleAction::new("check", None);

            action_check.connect_activate(clone!(@strong self.guess_entry as entry, @strong self.check_button as button => move |_, _| {
                let text = entry.buffer().text();
                button.set_label(&format!("Checking {}...", &text));
            }));

            self.obj().add_action(&action_check);
        }
    }

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}
