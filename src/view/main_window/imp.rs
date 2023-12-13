use std::cell::RefCell;
use std::rc::Rc;
use adw::gio::SimpleAction;
use adw::glib::{self, clone, MainContext};
use adw::subclass::prelude::*;
use glib::subclass::InitializingObject;
use gtk::{Button, CompositeTemplate, Entry};
use gtk::prelude::*;
use crate::model::{Trainer, TrainerEvent, Word};
use crate::observer::Observable;
use crate::view::web_image::WebImage;
use crate::view::word_list;

#[derive(CompositeTemplate, Default)]
#[template(resource = "/at/ac/tgm/pdamianik/spelling_trainer/main.ui")]
pub struct Window {
    #[template_child]
    pub image_view: TemplateChild<gtk::Box>,
    #[template_child]
    pub no_word: TemplateChild<gtk::Box>,
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

        let trainer = Rc::new(RefCell::new(Trainer::new(&[
            Word {
                url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Flogosmarcas.net%2Fwp-content%2Fuploads%2F2020%2F09%2FGoogle-Emblema.png&f=1&nofb=1&ipt=4401f1658a43f3414fb4bc3718df37b6a3422f3454f558de3f497bb39036517b&ipo=images".parse().unwrap(),
                word: "Google".to_string(),
            }
        ])));

        let web_image = WebImage::new();
        web_image.set_visible(false);
        self.image_view.append(&web_image);

        trainer.borrow_mut().observe(clone!(@strong web_image, @strong self.no_word as no_word => move |event| {
            match event {
                TrainerEvent::Selected(Some(ref word)) => {
                    web_image.set_visible(true);
                    no_word.set_visible(false);
                    let context = MainContext::default();
                    let url = word.url.clone();
                    context.spawn_local(clone!(@strong web_image => async move {
                        web_image.load(url).await.unwrap();
                    }));
                },
                TrainerEvent::Selected(None) => {
                    web_image.set_visible(false);
                    no_word.set_visible(true);
                },
                _ => {}
            }
        }));

        let action_random = SimpleAction::new("random", None);

        action_random.connect_activate(clone!(@strong trainer => move |_, _| {
            trainer.borrow_mut().random();
        }));
        obj.add_action(&action_random);

        let action_check = SimpleAction::new("check", None);

        action_check.connect_activate(clone!(@strong trainer, @strong self.guess_entry as entry, @strong self.check_button as button => move |_, _| {
            let text = entry.buffer().text();
            trainer.borrow_mut().guess(&text);
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
