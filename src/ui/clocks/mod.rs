mod imp;

use chrono::prelude::*;
use chrono::TimeZone;
use chrono_tz::Tz;
use gtk::{
    glib::{self, Object},
    Accessible, Buildable, ConstraintTarget, Widget,
};
use he::{prelude::*, Bin, ContentList, MiniContentBlock, TextButton};

glib::wrapper! {
    pub struct ClocksPage(ObjectSubclass<imp::ClocksPage>)
        @extends ContentList, Bin, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl ClocksPage {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClocksPage")
    }

    // temporary but I can't access libhelium code to make some better objects
    pub fn generate_row(location: gweather::Location) -> MiniContentBlock {
        let tz: Tz = location.timezone().unwrap().identifier().parse().unwrap();

        let block = MiniContentBlock::builder()
            .title(&location.city_name().unwrap().to_string())
            .subtitle(
                &tz.from_utc_datetime(&Utc::now().naive_utc())
                    .date()
                    .format("%a, %h %d") // Sun, Jun 19
                    .to_string(),
            )
            .primary_button(&TextButton::new(
                &tz.from_utc_datetime(&Utc::now().naive_utc())
                    .time()
                    .format("%H:%M:%S %p") // 09:34:02 AM
                    .to_string(),
            ))
            .build();

        return block;
    }

    pub fn fill(&self) {
        let loc = gweather::Location::new_detached("San Francisco", None, 37.773972, -122.431297);
        let block = ClocksPage::generate_row(loc);
        self.add(&block);
    }
}
