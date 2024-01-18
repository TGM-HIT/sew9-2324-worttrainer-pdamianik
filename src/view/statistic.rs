use adw::glib;
use adw::glib::{Cast, IsA, Object};
use adw::subclass::prelude::*;
use gtk::prelude::*;

glib::wrapper! {
    pub struct StatisticWindow(ObjectSubclass<imp::StatisticWindow>)
        @extends adw::Window, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root,
                    gtk::ShortcutManager;
}

impl StatisticWindow {
    pub fn new(application: &impl IsA<gtk::Application>, parent: &impl IsA<gtk::Window>) -> Self {
        let window = Object::builder::<StatisticWindow>()
            .property("application", application)
            .property("transient-for", parent)
            .build();
        window.update_statistic();
        window
    }

    pub fn update_statistic(&self) {
        let trainer = self.application()
            .expect("No application")
            .downcast::<crate::application::Application>()
            .expect("Could not downcast to custom application")
            .trainer();
        let statistic = trainer.borrow_mut().statistic().clone();

        let correct = statistic.correct();
        let incorrect = statistic.incorrect();
        let total = statistic.total();
        let percent = correct as f64 / total as f64 * 100.0;

        self.imp().statistic.set_text(&format!("{correct} correct, {incorrect} incorrect out of {total} ({percent:.2}%)"));
        self.action_set_enabled("app.reset", total != 0);
    }
}

mod imp {
    use adw::gdk::{Key, ModifierType};
    use adw::glib;
    use adw::subclass::prelude::*;
    use gtk::{CompositeTemplate, TemplateChild};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/statistic.ui")]
    pub struct StatisticWindow {
        #[template_child]
        pub statistic: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StatisticWindow {
        const NAME: &'static str = "StatisticWindow";
        type Type = super::StatisticWindow;
        type ParentType = adw::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.add_binding_action(Key::Escape, ModifierType::empty(), "window.close", None);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for StatisticWindow {}

    impl WidgetImpl for StatisticWindow {}

    impl WindowImpl for StatisticWindow {}

    impl AdwWindowImpl for StatisticWindow {}
}