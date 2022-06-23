mod imp {
    use gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::InitializingWidgetExt,
        subclass::prelude::*,
        CompositeTemplate,
    };
    use he::{subclass::prelude::*, Window};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/co/tauos/Nixie/clock_locations.ui")]
    pub struct ClockLocations {}

    #[glib::object_subclass]
    impl ObjectSubclass for ClockLocations {
        const NAME: &'static str = "NixieClockLocations";
        type Type = super::ClockLocations;
        type ParentType = Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl HeWindowImpl for ClockLocations {}
    impl WindowImpl for ClockLocations {}
    impl ObjectImpl for ClockLocations {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
    impl WidgetImpl for ClockLocations {}
}

use gtk::{
    gio::{ActionGroup, ActionMap},
    glib::{self, Object},
    Root, Widget,
};

glib::wrapper! {
    pub struct ClockLocations(ObjectSubclass<imp::ClockLocations>)
        @extends Widget, gtk::Window, he::Window, ActionMap, ActionGroup,
        @implements Root;
}

impl ClockLocations {
    pub fn new(parent: &crate::window::Window) -> Self {
        Object::new(&[("parent", parent)]).expect("Failed to create ClockLocations")
    }
}
