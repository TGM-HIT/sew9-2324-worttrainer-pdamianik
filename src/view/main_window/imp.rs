use adw::gio::SimpleAction;
use adw::glib::{self, clone, MainContext};
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{Button, CenterBox, CompositeTemplate, Entry};
use gtk::prelude::*;
use crate::view::web_image::WebImage;
use crate::view::word_list;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/main.ui")]
pub struct Window {
    #[template_child]
    pub image_view: TemplateChild<CenterBox>,
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

        let web_image = WebImage::new();
        let context = MainContext::default();
        context.spawn_local(clone!(@strong web_image => async move {
            web_image.load("https://source.unsplash.com/random").await.unwrap();
        }));
        self.image_view.set_center_widget(Some(&web_image));
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
