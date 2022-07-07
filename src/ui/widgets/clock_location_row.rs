use chrono::{Duration, Local, Offset, Utc};
use chrono_tz::Tz;
use gettextrs::ngettext;
use gtk::{
    glib::{wrapper, Object},
    subclass::prelude::*,
    Accessible, Actionable, Buildable, ConstraintTarget, Widget,
};
use gweather::{Location, LocationLevel};

mod imp {
    use std::cell::{Cell, RefCell};

    use gtk::{
        glib::{
            self, object_subclass, once_cell::sync::Lazy, subclass::InitializingObject, ParamFlags,
            ParamSpec, ParamSpecString, Value,
        },
        prelude::*,
        subclass::prelude::*,
        CompositeTemplate, ListBoxRow,
    };
    use gweather::Location;
    use log::debug;

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/clock_location_row.ui")]
    pub struct ClockLocationRow {
        pub location: Cell<Location>,
        pub clock_name: RefCell<Option<String>>,
        pub clock_location: RefCell<Option<String>>,
        pub clock_tz: RefCell<Option<String>>,
    }

    impl Default for ClockLocationRow {
        fn default() -> Self {
            Self {
                location: Cell::new(Location::world().unwrap()),
                clock_name: RefCell::new(None),
                clock_location: RefCell::new(None),
                clock_tz: RefCell::new(None),
            }
        }
    }

    #[object_subclass]
    impl ObjectSubclass for ClockLocationRow {
        const NAME: &'static str = "NixieClockLocationRow";
        type Type = super::ClockLocationRow;
        type ParentType = ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ListBoxRowImpl for ClockLocationRow {}
    impl ObjectImpl for ClockLocationRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("GtkListBoxRow<ClockLocationRow>::realize");
            });
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new("clock-name", "", "", None, ParamFlags::READWRITE),
                    ParamSpecString::new("clock-location", "", "", None, ParamFlags::READWRITE),
                    ParamSpecString::new("clock-tz", "", "", None, ParamFlags::READWRITE),
                ]
            });
            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "clock-name" => {
                    let p = value.get::<&str>().expect("Expected a string");
                    self.clock_name.replace(Some(p.to_string()));
                }
                "clock-location" => {
                    // This value should be optional
                    let p = value.get::<&str>().unwrap_or_default();
                    self.clock_location.replace(Some(p.to_string()));
                }
                "clock-tz" => {
                    let p = value.get::<&str>().expect("Expected a string");
                    self.clock_tz.replace(Some(p.to_string()));
                }
                _ => unimplemented!(),
            };
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "clock-name" => self
                    .clock_name
                    .try_borrow()
                    .unwrap()
                    .as_ref()
                    .map(|v| &v[..])
                    .to_value(),
                "clock-location" => self
                    .clock_location
                    .try_borrow()
                    .unwrap()
                    .as_ref()
                    .map(|v| &v[..])
                    .to_value(),
                "clock-tz" => self
                    .clock_tz
                    .try_borrow()
                    .unwrap()
                    .as_ref()
                    .map(|v| &v[..])
                    .to_value(),
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for ClockLocationRow {}
}

wrapper! {
    pub struct ClockLocationRow(ObjectSubclass<imp::ClockLocationRow>)
        @extends Widget,
        @implements Accessible, Buildable, ConstraintTarget, Actionable;
}

impl ClockLocationRow {
    pub fn new(loc: Location) -> Self {
        let obj: ClockLocationRow = Object::new(&[
            ("clock-name", &Self::parse_clock_name(loc.clone())),
            (
                "clock-location",
                &loc.clone().country_name().map(|v| String::from(v)),
            ),
            ("clock-tz", &Self::parse_clock_tz(loc.clone())),
        ])
        .expect("Failed to create ClockLocationRow");
        obj.imp().location.replace(loc);
        obj
    }

    fn get_state_name(loc: Location) -> Option<String> {
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
            "{}{}",
            loc.name().unwrap().to_string(),
            format!(
                "{}",
                Self::get_state_name(loc.clone()).unwrap_or(String::new())
            )
        );
    }

    fn parse_clock_tz(loc: Location) -> String {
        let tz: Tz = loc.timezone().unwrap().identifier().parse().unwrap();

        // Get timezone difference
        let local = Duration::seconds(
            (Local::now().offset().utc_minus_local()
                - Utc::now()
                    .with_timezone(&tz)
                    .offset()
                    .fix()
                    .utc_minus_local())
            .into(),
        )
        .num_hours();

        let mut message = String::from("Current timezone");

        if local > 0 {
            message = ngettext!(
                "{} hour earlier",
                "{} hours earlier",
                local.clone().try_into().unwrap(),
                local.clone().abs()
            );
        } else if local < 0 {
            message = ngettext!(
                "{} hour later",
                "{} hours later",
                local.clone().abs().try_into().unwrap(),
                local.clone().abs()
            );
        }

        return format!("{} • {}", &tz.to_string(), message.clone());
    }
}
