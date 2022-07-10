mod imp {
    use adw::{traits::ComboRowExt, ComboRow};
    use gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::InitializingWidgetExt,
        subclass::prelude::*,
        CompositeTemplate,
    };
    use he::{prelude::*, subclass::prelude::*, Window};
    use log::debug;

    use crate::duration::Duration;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/alarm_setup.ui")]
    pub struct AlarmSetup {
        #[template_child]
        pub ring_duration: TemplateChild<ComboRow>,
        #[template_child]
        pub snooze_duration: TemplateChild<ComboRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AlarmSetup {
        const NAME: &'static str = "NixieAlarmSetup";
        type Type = super::AlarmSetup;
        type ParentType = Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl HeWindowImpl for AlarmSetup {}
    impl WindowImpl for AlarmSetup {}
    impl ObjectImpl for AlarmSetup {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("HeWindow<AlarmSetup>::realize");
            });

            self.ring_duration.set_factory(Some(&Duration::factory()));
            self.ring_duration.set_model(Some(&Duration::model()));
            self.snooze_duration.set_factory(Some(&Duration::factory()));
            self.snooze_duration.set_model(Some(&Duration::model()));
        }
    }
    impl WidgetImpl for AlarmSetup {}
}

use gtk::{
    gio::{ActionGroup, ActionMap},
    glib::{self, Object},
    Root, Widget,
};

glib::wrapper! {
    pub struct AlarmSetup(ObjectSubclass<imp::AlarmSetup>)
        @extends Widget, gtk::Window, he::Window, ActionMap, ActionGroup,
        @implements Root;
}

impl AlarmSetup {
    pub fn new(parent: &crate::window::Window) -> Self {
        Object::new(&[("parent", parent)]).expect("Failed to create AlarmSetup")
    }
}
