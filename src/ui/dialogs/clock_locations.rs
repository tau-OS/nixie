mod imp {
    use gtk::{
        gio::ListStore,
        glib::{self, subclass::InitializingObject, Object},
        prelude::InitializingWidgetExt,
        subclass::prelude::*,
        CompositeTemplate, ListBox, ListBoxRow, ScrolledWindow, SearchEntry, Stack, Widget,
    };
    use gweather::Location;
    use he::{prelude::*, subclass::prelude::*, EmptyPage, Window};
    use log::debug;
    use unicode_casefold::UnicodeCaseFold;
    use unicode_normalization::UnicodeNormalization;

    use crate::{ui::widgets::clock_location_row::ClockLocationRow, clock_store::ClockStore};

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/clock_locations.ui")]
    pub struct ClockLocations {
        #[template_child]
        pub entry: TemplateChild<SearchEntry>,
        #[template_child]
        pub stack: TemplateChild<Stack>,

        #[template_child]
        pub empty_search: TemplateChild<EmptyPage>,
        #[template_child]
        pub results: TemplateChild<ScrolledWindow>,

        #[template_child]
        pub listbox: TemplateChild<ListBox>,

        pub locations: ListStore,
    }

    impl Default for ClockLocations {
        fn default() -> Self {
            Self {
                entry: TemplateChild::default(),
                stack: TemplateChild::default(),
                empty_search: TemplateChild::default(),
                results: TemplateChild::default(),
                listbox: TemplateChild::default(),
                locations: ListStore::new(Location::type_(&Location::world().unwrap())),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for ClockLocations {
        const NAME: &'static str = "NixieClockLocations";
        type Type = super::ClockLocations;
        type ParentType = Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl ClockLocations {
        #[template_callback]
        fn on_search_mode_notify(obj: gtk::SearchBar) {
            debug!("GtkSearchBar<ClockLocations>::search-mode-notify");
            if !obj.is_search_mode() {
                obj.set_search_mode(true);
            }
        }

        #[template_callback]
        fn on_search_changed(&self) {
            debug!("GtkSearchEntry<ClockLocations>::search-changed");
            // TODO This should be added to the hella based ContentList
            let mut fc = self.listbox.first_child();
            while fc != None {
                self.listbox.remove(&fc.unwrap());
                fc = self.listbox.first_child();
            }

            self.locations.remove_all();

            if self.entry.text() == "" {
                self.stack.set_visible_child(&self.empty_search.get());
                return;
            }

            let search: String = self
                .entry
                .text()
                .to_string()
                .nfd()
                .case_fold()
                .collect::<String>();
            let world = Location::world();
            if world == None {
                return;
            }

            let mut loc = super::ClockLocations::query_locations(&world.unwrap(), search);
            for location in loc.iter_mut() {
                self.locations.append(&location.clone().upcast::<Object>());
            }

            if self.locations.n_items() == 0 {
                self.stack.set_visible_child(&self.empty_search.get());
                return;
            }

            self.locations.sort(|a, b| {
                a.clone().downcast::<Location>()
                    .unwrap()
                    .sort_name()
                    .unwrap()
                    .cmp(
                        &b.clone().downcast::<Location>()
                            .unwrap()
                            .sort_name()
                            .unwrap(),
                    )
            });

            self.stack.set_visible_child(&self.results.get());
        }

        #[template_callback]
        fn on_item_activated (&self, _row: ListBoxRow) {
            let store = ClockStore::default();
            store.serialise_clocks(self.locations.clone());
        }
    }

    impl HeWindowImpl for ClockLocations {}
    impl WindowImpl for ClockLocations {}
    impl ObjectImpl for ClockLocations {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            self.entry
                .set_key_capture_widget(Some(&obj.upcast_ref::<Widget>().to_owned()));

            obj.connect_realize(move |_| {
                debug!("HeWindow<ClockLocations>::realize");
            });

            self.listbox
                .bind_model(Some(&self.locations), move |location| {
                    let crow = ClockLocationRow::new(
                        location.downcast_ref::<Location>().unwrap().to_owned(),
                    );

                    let row = ListBoxRow::builder().child(&crow).build();

                    return row.upcast_ref::<Widget>().to_owned();
                });
        }
    }
    impl WidgetImpl for ClockLocations {}
}

use gweather::{Location, LocationLevel};

use gtk::{
    gio::{ActionGroup, ActionMap},
    glib::{self, Object},
    Root, Widget,
};
use unicode_casefold::UnicodeCaseFold;
use unicode_normalization::UnicodeNormalization;

glib::wrapper! {
    pub struct ClockLocations(ObjectSubclass<imp::ClockLocations>)
        @extends Widget, gtk::Window, he::Window, ActionMap, ActionGroup,
        @implements Root;
}

const RESULT_COUNT_LIMIT: usize = 12;

impl ClockLocations {
    pub fn new(parent: &crate::window::Window) -> Self {
        Object::new(&[("parent", parent)]).expect("Failed to create ClockLocations")
    }

    fn query_locations(location: &Location, search: String) -> Vec<Location> {
        let mut new_loc: Vec<Location> = Vec::new();

        match &location.clone().level() {
            LocationLevel::City => {
                let contains_query = location.sort_name().unwrap().contains(&search);

                let mut country_name: String = "".to_string();
                if location.country_name() != None {
                    country_name = location
                        .country_name()
                        .unwrap()
                        .to_string()
                        .nfd()
                        .case_fold()
                        .collect::<String>();
                }

                let contains_country = country_name.contains(&search);

                if contains_query || contains_country {
                    new_loc.push(location.clone());
                }
            }
            LocationLevel::NamedTimezone => {
                // TODO handle is selected things i have no idea
                if location.sort_name().unwrap().contains(&search) {
                    new_loc.push(location.clone());
                }
            }
            _ => {}
        };

        let mut loc = location.next_child(None);
        while loc.clone() != None {
            new_loc.append(
                &mut super::clock_locations::ClockLocations::query_locations(
                    &loc.clone().unwrap(),
                    search.clone(),
                ),
            );
            if new_loc.len() >= RESULT_COUNT_LIMIT {
                return Vec::new();
            }

            loc = location.next_child(loc.as_ref());
        }

        return new_loc;
    }
}
