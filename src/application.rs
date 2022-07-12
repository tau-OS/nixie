use crate::{
    action,
    config::{APP_ID, PROFILE, VERSION},
    window::Window, clock::InternalClock,
};
use gtk::{
    gio::{self, ApplicationFlags, Settings},
    glib::{self, clone},
    prelude::*,
    subclass::prelude::*,
};
use he::{AboutWindow, AboutWindowLicenses};
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

    use crate::clock::InternalClock;

    pub struct Application {
        pub settings: Settings,
        pub clock: InternalClock,
    }

    impl Default for Application {
        fn default() -> Self {
            Self {
                settings: Settings::new("co.tauos.Nixie"),
                clock: InternalClock::new()
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = he::Application;
    }

    impl ObjectImpl for Application {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.setup_actions();

            obj.set_accels_for_action("app.quit", &["<primary>q"]);
            obj.set_accels_for_action("app.about", &["<primary>a"]);
        }
    }
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

    fn setup_actions(&self) {
        action!(
            self,
            "quit",
            clone!(@weak self as app => move |_, _| {
                app.quit()
            })
        );
        action!(
            self,
            "about",
            clone!(@weak self as app => move |_, _| {
                app.show_about();
            })
        )
    }

    pub fn settings(&self) -> Settings {
        self.imp().settings.clone()
    }

    pub fn clock(&self) -> InternalClock {
        self.imp().clock.clone()
    }

    fn create_window(&self) -> Window {
        Window::new(&self.clone())
    }

    pub fn run(&self) {
        info!("Nixie ({})", APP_ID);
        info!("Version: {} ({})", VERSION, PROFILE);

        ApplicationExtManual::run(self);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        AboutWindow::builder()
            .transient_for(&window)
            .modal(true)
            .icon_name(APP_ID)
            .app_name("Nixie")
            .version(VERSION)
            .developer_names(vec!["Jamie Murphy".into()])
            .copyright_year(2022)
            .license(AboutWindowLicenses::Gplv3)
            .build()
            .present();
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
