mod imp {
    use gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::InitializingWidgetExt,
        subclass::prelude::*,
        CompositeTemplate, ListBox, ListBoxRow, ScrolledWindow, SearchEntry, Stack,
    };
    use gweather::Location;
    use he::{prelude::*, subclass::prelude::*, EmptyPage, Window};
    use unicode_casefold::UnicodeCaseFold;
    use unicode_normalization::UnicodeNormalization;

    use crate::ui::dialogs::clock_location_row::ClockLocationRow;

    #[derive(CompositeTemplate, Default)]
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

        pub locations_vec: Vec<gweather::Location>,
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
            if !obj.is_search_mode() {
                obj.set_search_mode(true);
            }
        }

        #[template_callback]
        fn on_search_changed(&self) {
            // TODO This should be added to the hella based ContentList
            let mut fc = self.listbox.first_child();
            while fc != None {
                self.listbox.remove(&fc.unwrap());
                fc = self.listbox.first_child();
            }

            let mut locations = Vec::new();
            locations.clear();

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

            locations = super::ClockLocations::query_locations(&world.unwrap(), search);

            if locations.len() == 0 {
                self.stack.set_visible_child(&self.empty_search.get());
                return;
            }

            locations.sort_by(|a, b| a.sort_name().unwrap().cmp(&b.sort_name().unwrap()));

            for loc in locations.iter_mut() {
                let clock_row = &ClockLocationRow::new(loc.clone());

                let row = ListBoxRow::builder().child(clock_row).build();

                clock_row.connect_clicked(move |row| {
                    println!(
                        "Hello, {}",
                        row.location()
                            .name()
                            .unwrap_or(glib::GString::from("This Should Never Happen"))
                            .to_string()
                    );
                });

                self.listbox.append(&row)
            }

            self.listbox.connect_row_activated(move |_box, row| {
                row.first_child()
                    .unwrap()
                    .emit_by_name::<()>("clicked", &[]);
            });

            self.stack.set_visible_child(&self.results.get());
        }
    }

    impl HeWindowImpl for ClockLocations {}
    impl WindowImpl for ClockLocations {}
    impl ObjectImpl for ClockLocations {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
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
