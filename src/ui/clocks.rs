mod imp {
    use gtk::{
        glib::{self, subclass::InitializingObject},
        subclass::prelude::*,
        template_callbacks, CompositeTemplate, TemplateChild,
    };
    use he::{prelude::*, ContentList, OverlayButton};

    use crate::{ui::dialogs::clock_locations::ClockLocations, window::Window};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/co/tauos/Nixie/clocks.ui")]
    pub struct ClocksPage {
        #[template_child]
        pub btn: TemplateChild<OverlayButton>,
        #[template_child]
        pub list: TemplateChild<ContentList>,
    }

    #[template_callbacks]
    impl ClocksPage {
        #[template_callback]
        fn handle_btn_click(_button: &OverlayButton) {
            let dialog = ClockLocations::new(&Window::default());
            dialog.present();
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ClocksPage {
        const NAME: &'static str = "NixieClocksPage";
        type Type = super::ClocksPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl BoxImpl for ClocksPage {}
    impl ObjectImpl for ClocksPage {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.fill();
        }
    }
    impl WidgetImpl for ClocksPage {}
}

use gtk::{
    glib::{self, Object},
    subclass::prelude::*,
    Accessible, Box, Buildable, ConstraintTarget, Widget,
};
use he::prelude::*;

use super::clock_row::ClockRow;

glib::wrapper! {
    pub struct ClocksPage(ObjectSubclass<imp::ClocksPage>)
        @extends Box, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl ClocksPage {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClocksPage")
    }

    fn fill(&self) {
        let loc = gweather::Location::new_detached("San Francisco", None, 37.773972, -122.431297);
        let block = ClockRow::new();
        block.setup_row(loc);

        self.imp().list.add(&block);
    }
}
