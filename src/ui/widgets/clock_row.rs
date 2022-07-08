use chrono::{Offset, TimeZone, Utc};
use chrono_tz::Tz;
use gtk::{
    glib::{self, Object},
    subclass::prelude::*,
    ListBoxRow, Widget,
};
use gweather::{Location, LocationLevel};
use he::prelude::*;

mod imp {
    use std::cell::{Cell, RefCell};

    use chrono::{NaiveDateTime, TimeZone, Utc};
    use chrono_tz::Tz;
    use gtk::{
        glib::{
            self, clone, once_cell::sync::Lazy, subclass::*, timeout_add_local, ParamFlags,
            ParamSpec, ParamSpecInt64, ParamSpecString, Value,
        },
        subclass::prelude::*,
        CompositeTemplate, Label, ListBoxRow,
    };
    use gweather::Location;
    use he::prelude::*;
    use log::debug;

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/clock_row.ui")]
    pub struct ClockRow {
        pub location: Cell<Location>,
        pub clock_name: RefCell<String>,
        pub clock_desc: RefCell<String>,
        pub clock_time: Cell<NaiveDateTime>,

        #[template_child]
        pub time_label: TemplateChild<Label>,
    }

    impl Default for ClockRow {
        fn default() -> Self {
            Self {
                location: Cell::new(Location::world().unwrap()),
                clock_name: RefCell::new(String::default()),
                clock_desc: RefCell::new(String::default()),
                clock_time: Cell::new(Utc::now().naive_utc()),
                time_label: TemplateChild::default(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ClockRow {
        const NAME: &'static str = "NixieClockRow";
        type Type = super::ClockRow;
        type ParentType = ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ListBoxRowImpl for ClockRow {}
    impl ObjectImpl for ClockRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("HeMiniContentBlock<ClockRow>::realize");
            });

            timeout_add_local(
                std::time::Duration::from_millis(1),
                clone!(@weak obj => @default-return Continue(false), move || {
                    let tz: Tz = obj.location().timezone().unwrap().identifier().parse().unwrap();
                    obj.imp().time_label.set_label(&tz.from_utc_datetime(&Utc::now().naive_utc())
                    .time()
                    .format("%H:%M:%S %p") // 09:34:02 AM
                    .to_string());
                    Continue(true)
                }),
            );
        }

        fn properties() -> &'static [glib::ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new("clock-name", "", "", None, ParamFlags::READWRITE),
                    ParamSpecString::new("clock-desc", "", "", None, ParamFlags::READWRITE),
                    ParamSpecInt64::new(
                        "clock-time",
                        "",
                        "",
                        i64::MIN,
                        i64::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "clock-name" => {
                    let p = value.get::<&str>().expect("Expected a string");
                    self.clock_name.replace(p.to_string());
                }
                "clock-desc" => {
                    let p = value.get::<&str>().expect("Expected a string");
                    self.clock_desc.replace(p.to_string());
                }
                "clock-time" => {
                    let p = value.get::<i64>().expect("Expected a n i64");
                    self.clock_time.replace(NaiveDateTime::from_timestamp(p, 0));
                }
                _ => unimplemented!(),
            };
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "clock-name" => self
                    .clock_name
                    .try_borrow()
                    .expect("Expected a string")
                    .to_value(),
                "clock-desc" => self
                    .clock_desc
                    .try_borrow()
                    .expect("Expected a string")
                    .to_value(),
                "clock-time" => self.clock_time.get().timestamp().to_value(),
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for ClockRow {}
}

glib::wrapper! {
    pub struct ClockRow(ObjectSubclass<imp::ClockRow>)
        @extends Widget, ListBoxRow;
}

impl ClockRow {
    pub fn new(loc: Location) -> Self {
        let tz: Tz = loc.timezone().unwrap().identifier().parse().unwrap();
        let time = &tz.from_utc_datetime(&Utc::now().naive_utc());
        let obj: ClockRow = Object::new(&[
            ("clock-name", &Self::parse_clock_name(loc.clone())),
            ("clock-desc", &Self::parse_clock_desc(loc.clone())),
            ("clock-time", &time.timestamp().to_value()),
        ])
        .expect("Failed to create ClockRow");
        obj.imp().location.replace(loc);
        obj
    }

    pub fn location(&self) -> Location {
        let loc = self.imp().location.replace(Location::world().unwrap());
        self.imp().location.replace(loc.clone());
        return loc;
    }

    fn state_name(loc: Location) -> Option<String> {
        let mut top_loc = loc.parent().unwrap();
        while top_loc.clone().level() != LocationLevel::Adm1 {
            if top_loc.clone().parent() == None {
                return None;
            } else {
                top_loc = top_loc.parent().unwrap();
            }
        }

        return Some(format!(", {}", top_loc.name().unwrap().to_string()));
    }

    fn parse_clock_name(loc: Location) -> String {
        return format!(
            "{}{}{}",
            loc.name().unwrap().to_string(),
            format!("{}", Self::state_name(loc.clone()).unwrap_or(String::new())),
            format!(
                "{}",
                &loc.clone()
                    .country_name()
                    .map(|v| String::from(format!(", {}", v)))
                    .unwrap_or(String::from(""))
            )
        );
    }

    fn parse_clock_desc(loc: Location) -> String {
        let tz: Tz = loc.timezone().unwrap().identifier().parse().unwrap();

        let local = chrono::Duration::seconds(
            (Utc::now()
                .with_timezone(&tz)
                .offset()
                .fix()
                .local_minus_utc())
            .into(),
        )
        .num_hours();

        let mut identifier = String::from("");

        if local > 0 {
            identifier = format!("+{}", local);
        } else if local == 0 {
            identifier = String::from("");
        } else if local < 0 {
            identifier = local.to_string();
        }

        return format!("UTC{}", identifier);
    }
}
