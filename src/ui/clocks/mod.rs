mod imp;

use gtk::{
    glib::{self, Object},
    Accessible, Buildable, ConstraintTarget, Widget,
};
use he::{prelude::*, Bin, ContentList};

use super::clock_row::generate_row;

glib::wrapper! {
    pub struct ClocksPage(ObjectSubclass<imp::ClocksPage>)
        @extends ContentList, Bin, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl ClocksPage {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClocksPage")
    }

    pub fn fill(&self) {
        let loc = gweather::Location::new_detached("San Francisco", None, 37.773972, -122.431297);
        let block = generate_row(loc);
        self.add(&block);
    }
}
