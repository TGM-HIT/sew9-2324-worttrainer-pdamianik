use adw::glib;
use adw::glib::subclass::InitializingObject;
use adw::subclass::prelude::*;
use gtk::CompositeTemplate;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/word_list.ui")]
pub struct Window {

}

#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "SpellingTrainerWordListWindow";
    type Type = super::Window;
    type ParentType = adw::Window;

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
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl AdwWindowImpl for Window {}