use gtk::{
    glib::{self, subclass::InitializingObject},
    subclass::prelude::*,
    CompositeTemplate,
};
use he::{prelude::*, subclass::prelude::*};

#[derive(CompositeTemplate, Default)]
#[template(file = "clocks.ui")]
pub struct ClocksPage {}

#[glib::object_subclass]
impl ObjectSubclass for ClocksPage {
    const NAME: &'static str = "NixieClocksPage";
    type Type = super::ClocksPage;
    type ParentType = he::ContentList;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BinImpl for ClocksPage {}
impl ContentListImpl for ClocksPage {}
impl ObjectImpl for ClocksPage {
    fn constructed(&self, obj: &Self::Type) {
        self.parent_constructed(obj);
    }
}
impl WidgetImpl for ClocksPage {}
