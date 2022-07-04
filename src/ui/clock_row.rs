use std::{thread, time::Duration};

use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use gtk::{
    glib::{self, clone, MainContext, Object, PRIORITY_DEFAULT},
    Accessible, Buildable, ConstraintTarget, Widget,
};
use gweather::{Location, LocationLevel};
use he::{prelude::*, Bin, MiniContentBlock, TextButton};

mod imp {
    use gtk::{glib, subclass::prelude::*};
    use he::{subclass::prelude::*, MiniContentBlock, prelude::*};
    use log::debug;

    #[derive(Default)]
    pub struct ClockRow {}

    #[glib::object_subclass]
    impl ObjectSubclass for ClockRow {
        const NAME: &'static str = "ClockRow";
        type Type = super::ClockRow;
        type ParentType = MiniContentBlock;
    }

    impl MiniContentBlockImpl for ClockRow {}
    impl BinImpl for ClockRow {}
    impl ObjectImpl for ClockRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("HeMiniContentBlock<ClockRow>::realize");
            });
        }
    }
    impl WidgetImpl for ClockRow {}
}

glib::wrapper! {
    pub struct ClockRow(ObjectSubclass<imp::ClockRow>)
        @extends MiniContentBlock, Bin, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl ClockRow {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClockRow")
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
        let time = &tz.from_utc_datetime(&Utc::now().naive_utc());

        let time_label = &TextButton::new(
            &time
                .time()
                .format("%H:%M:%S %p") // 09:34:02 AM
                .to_string(),
        );

        self.set_title(&format!(
            "{}{}",
            &loc.name().unwrap().to_string(),
            Self::get_state_name(loc.clone()).unwrap_or(String::new())
        ));
        self.set_subtitle(
            &time
                .date()
                .format("%a, %h %d") // Sun, Jun 19
                .to_string(),
        );
        self.set_primary_button(time_label);

        let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

        thread::spawn(move || loop {
            sender.send(true).expect("Could not send through channel");
            thread::sleep(Duration::from_millis(1));
        });

        let _self = self;

        receiver.attach(
            None,
            clone!(@weak _self => @default-return Continue(false),
            move |_not_used| {
                _self.set_primary_button(&TextButton::new(
                    &tz.from_utc_datetime(&Utc::now().naive_utc())
                        .time()
                        .format("%H:%M:%S %p") // 09:34:02 AM
                        .to_string(),
                ));

                Continue(true)
            }),
        );
    }
}
