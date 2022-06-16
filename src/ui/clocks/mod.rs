mod imp;

use gtk::{
    glib::{self, Object},
    Accessible, Buildable, ConstraintTarget, Widget,
};
use he::{ContentList, Bin};

glib::wrapper! {
    pub struct ClocksPage(ObjectSubclass<imp::ClocksPage>)
        @extends ContentList, Bin, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl ClocksPage {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClocksPage")
    }
}
