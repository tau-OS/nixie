mod imp {
    use chrono_tz::Tz;
    use gtk::{
        glib::{self, subclass::InitializingObject},
        prelude::InitializingWidgetExt,
        subclass::prelude::*,
        CompositeTemplate, SearchEntry, Stack,
    };
    use gweather::Location;
    use he::{prelude::*, subclass::prelude::*, ContentList, MiniContentBlock, Window};
    use unicode_casefold::UnicodeCaseFold;
    use unicode_normalization::UnicodeNormalization;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/co/tauos/Nixie/clock_locations.ui")]
    pub struct ClockLocations {
        #[template_child]
        pub entry: TemplateChild<SearchEntry>,
        #[template_child]
        pub stack: TemplateChild<Stack>,
        #[template_child]
        pub listbox: TemplateChild<ContentList>,

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
            let mut locations = self.locations_vec.clone();
            locations.clear();

            if self.entry.text() == "" {
                self.stack.set_visible_child_name("empty_search");
                return;
            }

            // TODO normalise
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

            locations = super::ClockLocations::query_locations(locations, &world.unwrap(), search);

            if locations.len() == 0 {
                self.stack.set_visible_child_name("empty_search");
                return;
            }

            locations.sort_by(|a, b| a.sort_name().unwrap().cmp(&b.sort_name().unwrap()));

            for loc in locations.iter_mut() {
                let tz: Tz = loc.timezone().unwrap().identifier().parse().unwrap();
                self.listbox.add(
                    &MiniContentBlock::builder()
                        .title(&loc.city_name().unwrap().to_string())
                        .subtitle(&tz.to_string())
                        .build(),
                )
            }

            self.stack.set_visible_child_name("results");
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

    fn query_locations(
        locations: Vec<Location>,
        location: &Location,
        search: String,
    ) -> Vec<Location> {
        if locations.len() >= RESULT_COUNT_LIMIT {
            return Vec::new();
        } else {
            let mut new_loc: Vec<Location> = locations.clone();

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
                super::clock_locations::ClockLocations::query_locations(
                    locations.clone(),
                    &loc.clone().unwrap(),
                    search.clone(),
                );
                if locations.len() >= RESULT_COUNT_LIMIT {
                    return Vec::new();
                }
                loc = location.next_child(loc.as_ref());
            }

            return new_loc;
        }
    }
}
