mod imp {
    use crate::{alarm::Alarm, weekday::NixieWeekdays};
    use chrono::{Datelike, Local, Timelike};
    use gtk::{
        glib::{
            self, once_cell::sync::Lazy, subclass::InitializingObject, ParamFlags, ParamSpec,
            ParamSpecBoolean, ParamSpecString, ParamSpecUInt, Value,
        },
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

        pub alarm: Alarm,
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

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new("alarm-name", "", "", Some(""), ParamFlags::READWRITE),
                    ParamSpecUInt::new(
                        "alarm-hour",
                        "",
                        "",
                        u32::MIN,
                        u32::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecUInt::new(
                        "alarm-minute",
                        "",
                        "",
                        u32::MIN,
                        u32::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new("alarm-ring", "", "", false, ParamFlags::READWRITE),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "alarm-name" => {
                    self.alarm
                        .name
                        .replace(value.get::<String>().expect("Failed to get string value"));
                }
                "alarm-hour" => {
                    self.alarm.time.replace(
                        self.alarm
                            .time
                            .get()
                            .with_hour(value.get::<u32>().expect("Failed to get integer value"))
                            .unwrap(),
                    );
                }
                "alarm-minute" => {
                    self.alarm.time.replace(
                        self.alarm
                            .time
                            .get()
                            .with_minute(value.get::<u32>().expect("Failed to get integer value"))
                            .unwrap(),
                    );
                }
                "alarm-ring" => {
                    self.alarm
                        .ring
                        .replace(value.get::<bool>().expect("Failed to get boolean value"));
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "alarm-name" => self.alarm.name.try_borrow().unwrap().to_value(),
                "alarm-hour" => self.alarm.time.get().time().hour().to_value(),
                "alarm-minute" => self.alarm.time.get().time().minute().to_value(),
                "alarm-ring" => self.alarm.ring.get().to_value(),
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for AlarmSetup {}
}

use chrono::Timelike;
use gtk::{
    gio::{ActionGroup, ActionMap},
    glib::{self, Object},
    prelude::ToValue,
    Root, Widget,
};

use crate::alarm::Alarm;

glib::wrapper! {
    pub struct AlarmSetup(ObjectSubclass<imp::AlarmSetup>)
        @extends Widget, gtk::Window, he::Window, ActionMap, ActionGroup,
        @implements Root;
}

impl AlarmSetup {
    pub fn new(parent: &crate::window::Window, alarm: Option<Alarm>) -> Self {
        Object::new(&[
            ("parent", parent),
            (
                "alarm-name",
                &alarm
                    .clone()
                    .unwrap_or(Alarm::default())
                    .name
                    .try_borrow()
                    .unwrap()
                    .to_value(),
            ),
            (
                "alarm-hour",
                &alarm
                    .clone()
                    .unwrap_or(Alarm::default())
                    .time
                    .get()
                    .hour()
                    .to_value(),
            ),
            (
                "alarm-minute",
                &alarm
                    .clone()
                    .unwrap_or(Alarm::default())
                    .time
                    .get()
                    .minute()
                    .to_value(),
            ),
            (
                "alarm-ring",
                &alarm
                    .clone()
                    .unwrap_or(Alarm::default())
                    .ring
                    .get()
                    .to_value(),
            ),
        ])
        .expect("Failed to create AlarmSetup")
    }
}
