use gtk::{
    gio::{ActionGroup, ActionMap},
    glib::{self, Object}, ApplicationWindow, Widget, Root,
};

use crate::Application;

mod imp {
    use gtk::{
        glib::{self, subclass::InitializingObject},
        subclass::prelude::*,
        CompositeTemplate,
    };
    use he::{prelude::*, subclass::prelude::*, ApplicationWindow};
    use log::debug;

    use crate::{ui::{clocks::ClocksPage, stopwatch::StopwatchPage, alarms::AlarmsPage}, config::APP_ID};

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/window.ui")]
    pub struct Window {}

    impl Default for Window {
        fn default() -> Self {
            Self {}
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "NixieWindow";
        type Type = super::Window;
        type ParentType = ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            ClocksPage::ensure_type();
            StopwatchPage::ensure_type();
            AlarmsPage::ensure_type();

            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl HeApplicationWindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl WindowImpl for Window {
        fn close_request(&self, window: &Self::Type) -> gtk::Inhibit {
            // Pass close request on to the parent
            self.parent_close_request(window)
        }

    }
    impl ObjectImpl for Window {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("HeWindow<Window>::realize");
            });

            if APP_ID.ends_with("Devel") {
                obj.add_css_class("devel");
            }
        }
    }
    impl WidgetImpl for Window {}
}

glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends Widget, gtk::Window, ApplicationWindow, he::ApplicationWindow, ActionMap, ActionGroup,
        @implements Root;

}

impl Window {
    pub fn new(app: &Application) -> Self {
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
}

impl Default for Window {
    fn default() -> Self {
        Window::new(&Application::default())
    }
}
