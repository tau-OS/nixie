use crate::{window::Window, config::{VERSION, PROFILE, APP_ID}, action};
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

// Simple macro to make actions easier :)
#[macro_export]
macro_rules! action {
    ($actions_group:expr, $name:expr, $callback:expr) => {
        {
            let simple_action = gio::SimpleAction::new($name, None);
            simple_action.connect_activate($callback);
            $actions_group.add_action(&simple_action);
        }
    };
    ($actions_group:expr, $name:expr, $param_type:expr, $callback:expr) => {
        {
            let simple_action = gio::SimpleAction::new($name, $param_type);
            simple_action.connect_activate($callback);
            $actions_group.add_action(&simple_action);
        }
    };
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
