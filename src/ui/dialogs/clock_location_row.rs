use chrono_tz::Tz;
use gtk::{
    glib::{self, clone, Object, SignalHandlerId},
    Accessible, Buildable, ConstraintTarget, Widget,
};
use gweather::{Location, LocationLevel};
use he::{prelude::*, subclass::prelude::*, Bin, MiniContentBlock};

mod imp {
    use gtk::{
        glib::{self, once_cell::sync::Lazy, subclass::Signal},
        subclass::prelude::*,
    };
    use gweather::Location;
    use he::{prelude::*, subclass::prelude::*, MiniContentBlock};
    use std::cell::Cell;

    #[derive(Default)]
    pub struct ClockLocationRow {
        // yes its a vector, Default isn't implemented idk why
        pub location: Cell<Vec<Location>>,
    }

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
        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("clicked", &[], <()>::static_type().into())
                    .action()
                    .build()]
            });
            SIGNALS.as_ref()
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
    pub fn new(loc: Location) -> Self {
        let obj: ClockLocationRow = Object::new(&[]).expect("Failed to create ClockLocationRow");
        obj.imp().location.replace(vec![loc.clone()]);
        obj.setup_row(loc);
        obj
    }

    pub fn connect_clicked<F>(&self, callback: F) -> SignalHandlerId
    where
        F: Fn(&Self) + 'static,
    {
        self.connect_local(
            "clicked",
            false,
            clone!(@weak self as row => @default-return None, move |_| {
                callback(&row);
                None
            }),
        )
    }

    pub fn location(&self) -> Location {
        return self
            .imp()
            .location
            .take()
            .get(0)
            // TODO: should be able to use .get instead of .take. hell, i shouldn't even need a Vector. location moment.
            .unwrap_or(&Location::world().unwrap())
            .clone();
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

    fn setup_row(&self, loc: Location) {
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
