use crate::{window::Window, config::{VERSION, PROFILE, APP_ID}};
use gtk::{
    gio::{self, ApplicationFlags, Settings},
    glib,
    prelude::*,
    subclass::prelude::*,
};
use log::info;

mod imp {
    use gtk::{
        gio::Settings,
        glib,
        subclass::prelude::{ApplicationImplExt, GtkApplicationImpl, ObjectImpl, ObjectSubclass},
    };
    use he::prelude::*;
    use he::subclass::prelude::*;
    use log::debug;

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
            debug!("HeApplication<Application>::activate");
            let window = app.create_window();
            window.present();
        }

        fn startup(&self, app: &Self::Type) {
            debug!("HeApplication<Application>::startup");
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

    pub fn run(&self) {
        info!("Nixie ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);

        ApplicationExtManual::run(self);
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
