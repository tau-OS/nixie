mod imp {
    use gtk::{
        glib::{self, subclass::InitializingObject},
        subclass::prelude::*,
        CompositeTemplate,
    };
    use he::prelude::*;
    use log::debug;

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/alarms.ui")]
    pub struct AlarmsPage {}

    impl Default for AlarmsPage {
        fn default() -> Self {
            Self {}
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AlarmsPage {
        const NAME: &'static str = "NixieAlarmsPage";
        type Type = super::AlarmsPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl BoxImpl for AlarmsPage {}
    impl ObjectImpl for AlarmsPage {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("GtkBox<AlarmsPage>::realize");
            });
        }
    }
    impl WidgetImpl for AlarmsPage {}
}

use gtk::{
    glib::{self, Object},
    Accessible, Box, Buildable, ConstraintTarget, Widget,
};

glib::wrapper! {
    pub struct AlarmsPage(ObjectSubclass<imp::AlarmsPage>)
        @extends Box, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl AlarmsPage {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create Alarms")
    }
}
