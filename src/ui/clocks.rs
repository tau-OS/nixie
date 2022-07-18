mod imp {
    use gtk::{
        gdk::CURRENT_TIME,
        gio::ListStore,
        glib::{self, subclass::InitializingObject, TimeType},
        subclass::prelude::*,
        template_callbacks, CompositeTemplate, ListBox, Stack, TemplateChild, Widget,
    };
    use gweather::Location;
    use he::{prelude::*, EmptyPage, OverlayButton};
    use log::debug;

    use crate::{
        application::Application,
        clock_store::ClockStore,
        ui::{dialogs::clock_locations::ClockLocations, widgets::clock_row::ClockRow},
    };

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/clocks.ui")]
    pub struct ClocksPage {
        #[template_child]
        pub btn: TemplateChild<OverlayButton>,
        #[template_child]
        pub list: TemplateChild<ListBox>,
        #[template_child]
        pub empty: TemplateChild<EmptyPage>,
        #[template_child]
        pub stack: TemplateChild<Stack>,

        pub clocks: ListStore,
    }

    impl Default for ClocksPage {
        fn default() -> Self {
            Self {
                btn: TemplateChild::default(),
                list: TemplateChild::default(),
                empty: TemplateChild::default(),
                stack: TemplateChild::default(),
                clocks: ListStore::new(Location::type_(&Location::world().unwrap())),
            }
        }
    }

    #[template_callbacks]
    impl ClocksPage {
        #[template_callback]
        fn handle_btn_click(&self) {
            debug!("HeOverlayButton<ClocksPage>::clicked");
            let dialog = ClockLocations::new(&Application::main_window(&Application::default()));

            let _self = self.clocks.clone();
            dialog.connect_location_added(move |_, loc| {
                _self.append(&loc);
                ClockStore::default().serialise_clocks(_self.clone());
            });

            dialog.present();
        }

        fn load_data(&self) {
            for loc in ClockStore::default().deserialize_clocks() {
                self.clocks.append(&loc);
            }
        }

        fn load_view(&self) {
            if self.clocks.n_items() <= 0 {
                self.stack
                    .set_visible_child(self.empty.upcast_ref::<Widget>())
            } else {
                self.stack
                    .set_visible_child(self.list.upcast_ref::<Widget>())
            }
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

            obj.connect_realize(move |_| {
                debug!("GtkBox<ClocksPage>::realize");
            });

            self.clocks.sort(move |p1, p2| {
                let l1 = p1.downcast_ref::<Location>().unwrap();
                let l2 = p2.downcast_ref::<Location>().unwrap();

                let o1 = l1.timezone().unwrap().offset(
                    l1.timezone()
                        .unwrap()
                        .find_interval(TimeType::Universal, CURRENT_TIME.into()),
                );
                let o2 = l2.timezone().unwrap().offset(
                    l2.timezone()
                        .unwrap()
                        .find_interval(TimeType::Universal, CURRENT_TIME.into()),
                );

                return o1.cmp(&o2);
            });

            self.list.bind_model(Some(&self.clocks), move |loc| {
                let row = ClockRow::new(loc.downcast_ref::<Location>().unwrap().to_owned());
                return row.upcast_ref::<Widget>().to_owned();
            });

            let _self = obj.clone();
            self.clocks.connect_items_changed(move |_, _, _, _| {
                ClockStore::default().serialise_clocks(_self.imp().clocks.clone());
                _self.imp().load_view();
            });

            self.load_data();
            self.load_view();
        }
    }
    impl WidgetImpl for ClocksPage {}
}

use gtk::{
    glib::{self, Object},
    Accessible, Box, Buildable, ConstraintTarget, Widget,
};

glib::wrapper! {
    pub struct ClocksPage(ObjectSubclass<imp::ClocksPage>)
        @extends Box, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl ClocksPage {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClocksPage")
    }
}
