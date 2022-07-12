mod imp {
    use crate::weekday::NixieWeekdays;
    use chrono::{Datelike, Local};
    use gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::InitializingWidgetExt,
        subclass::prelude::*,
        Align, Box, CompositeTemplate, Entry, Switch, ToggleButton,
    };
    use he::{prelude::*, subclass::prelude::*, Window};
    use log::debug;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/alarm_setup.ui")]
    pub struct AlarmSetup {
        #[template_child]
        pub alarm_label_entry: TemplateChild<Entry>,
        #[template_child]
        pub alarm_ringer_switch: TemplateChild<Switch>,
        #[template_child]
        pub repeat_box: TemplateChild<Box>,
    }

    impl AlarmSetup {
        fn setup_repeats(&self) {
            let weekday = Local::now().weekday();

            for day in weekday.iterator() {
                let btn = ToggleButton::builder()
                    .label(&day.symbol())
                    .tooltip_text(&day.text(0))
                    .css_classes(vec!["circular".to_string()])
                    .halign(Align::Start)
                    .build();

                self.repeat_box.append(&btn);
            }
        }
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

            self.setup_repeats();
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
