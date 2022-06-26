use chrono_tz::Tz;
use gtk::{
    glib::{self, Object},
    Accessible, Buildable, ConstraintTarget, Widget
};
use gweather::{Location, LocationLevel};
use he::{prelude::*, Bin, MiniContentBlock};

mod imp {
    use gtk::{glib, subclass::prelude::*};
    use he::{subclass::prelude::*, MiniContentBlock};

    #[derive(Default)]
    pub struct ClockLocationRow {}

    #[glib::object_subclass]
    impl ObjectSubclass for ClockLocationRow {
        const NAME: &'static str = "ClockLocationRow";
        type Type = super::ClockLocationRow;
        type ParentType = MiniContentBlock;
    }

    impl MiniContentBlockImpl for ClockLocationRow {}
    impl BinImpl for ClockLocationRow {}
    impl ObjectImpl for ClockLocationRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
        }
    }
    impl WidgetImpl for ClockLocationRow {}
}

glib::wrapper! {
    pub struct ClockLocationRow(ObjectSubclass<imp::ClockLocationRow>)
        @extends MiniContentBlock, Bin, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl ClockLocationRow {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClockLocationRow")
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

    pub fn setup_row(&self, loc: Location) {
        let tz: Tz = loc.timezone().unwrap().identifier().parse().unwrap();

        self.set_title(&format!(
            "{}{}",
            &loc.name().unwrap().to_string(),
            Self::get_state_name(loc.clone()).unwrap_or(String::new())
        ));
        self.set_subtitle(
            &loc.country_name()
                .unwrap_or(glib::GString::from(tz.to_string())),
        );
    }
}
