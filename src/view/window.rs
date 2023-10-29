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
    use glib::subclass::InitializingObject;
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use adw::glib;
    use adw::subclass::prelude::*;
    use gtk::{Button, CompositeTemplate};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/window.ui")]
    pub struct Window {
        #[template_child]
        pub button: TemplateChild<Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "MyAdwAppWindow";
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

            self.button.connect_clicked(move |button| {
                button.set_label("Hello World!");
            });
        }
    }

    impl WidgetImpl for Window {}

    impl WindowImpl for Window {}

    impl ApplicationWindowImpl for Window {}

    impl AdwApplicationWindowImpl for Window {}
}
