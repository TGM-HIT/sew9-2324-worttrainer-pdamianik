mod imp;

use adw::glib;
use glib::Object;
use gtk::prelude::*;

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends adw::Window, gtk::Window, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(parent: &impl IsA<gtk::Window>) -> Self {
        Object::builder()
            .property("transient-for", parent.clone().upcast())
            .build()
    }
}
