use gtk::{
    gio::{self, ApplicationFlags, Settings},
    glib::{self, set_application_name},
    prelude::*,
    subclass::prelude::*,
};
use window::Window;

mod ui;
mod window;

mod imp {
    use gtk::{
        gio::{Settings, self},
        glib,
        subclass::prelude::{ApplicationImplExt, GtkApplicationImpl, ObjectImpl, ObjectSubclass},
    };
    use he::subclass::prelude::*;
    use he::prelude::*;

    pub struct Application {
        pub settings: Settings,
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
                settings: Settings::new("co.tauos.Nixie"),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = he::Application;
    }

    impl ObjectImpl for Application {}
    impl ApplicationImpl for Application {
        fn activate(&self, app: &Self::Type) {
            gio::resources_register_include!("co.tauos.Nixie.gresource")
                .expect("Failed to register resources");

            let window = app.create_window();
            window.present();
        }

        fn startup(&self, app: &Self::Type) {
            self.parent_startup(app);
        }
    }
    impl GtkApplicationImpl for Application {}
    impl HeApplicationImpl for Application {}
}

glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application, he::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}

impl Application {
    pub fn new() -> Self {
        glib::Object::new(&[
            ("application-id", &Some("co.tauos.Nixie")),
            ("flags", &ApplicationFlags::default()),
        ])
        .expect("Failed to create Application")
    }

    pub fn settings(&self) -> Settings {
        self.imp().settings.clone()
    }

    fn create_window(&self) -> Window {
        Window::new(&self.clone())
    }
}

impl Default for Application {
    fn default() -> Self {
        gio::Application::default()
            .unwrap()
            .downcast::<Application>()
            .unwrap()
    }
}

fn main() {
    set_application_name("Nixie");

    let app = Application::new();
    app.run();
}
