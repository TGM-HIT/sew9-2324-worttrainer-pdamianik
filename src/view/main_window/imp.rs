use std::cell::RefCell;
use std::rc::Rc;
use adw::gio::SimpleAction;
use crate::view::web_image::load_image;
use adw::glib::{self, clone, MainContext};
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{Button, CompositeTemplate, Entry, Image, Spinner};
use gtk::prelude::*;
use crate::view::word_list;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/main.ui")]
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
        let obj = self.obj();
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

        obj.add_action(&action_check);

        let action_edit = SimpleAction::new("edit", None);

        {
            let obj = obj.clone();
            action_edit.connect_activate(move |_, _| {
                let edit_window = word_list::Window::new(&obj);
                edit_window.present();
            });
        }

        obj.add_action(&action_edit);
    }
}

impl WidgetImpl for Window {}

impl WindowImpl for Window {}

impl ApplicationWindowImpl for Window {}

impl AdwApplicationWindowImpl for Window {}
