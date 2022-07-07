use gtk::{
    gio::ListStore,
    glib::{wrapper, FromVariant, Object, Variant},
    prelude::*,
    subclass::prelude::*,
};
use gweather::Location;

mod imp {
    use gtk::{
        gio::Settings,
        glib::{self, object_subclass},
        subclass::prelude::*,
    };

    use crate::application::Application;

    pub struct ClockStore {
        pub settings: Settings,
    }

    impl Default for ClockStore {
        fn default() -> Self {
            Self {
                settings: Application::settings(&Application::default()),
            }
        }
    }

    #[object_subclass]
    impl ObjectSubclass for ClockStore {
        const NAME: &'static str = "ClockStore";
        type Type = super::ClockStore;
    }

    impl ObjectImpl for ClockStore {}
}

wrapper! {
    pub struct ClockStore(ObjectSubclass<imp::ClockStore>);
}

impl Default for ClockStore {
    fn default() -> Self {
        ClockStore::new()
    }
}

impl ClockStore {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create ClockStore")
    }

    pub fn serialise_clocks(&self, clock_list: ListStore) {
        let variant = clock_list
            .into_iter()
            .map(|v| v.unwrap().downcast_ref::<Location>().unwrap().serialize())
            .collect::<Vec<Variant>>()
            .to_variant();

        self.imp()
            .settings
            .clone()
            .set_value("clocks", &variant)
            .expect("Failed to save Clock data");
    }

    pub fn deserialize_clocks(&self) -> Vec<Location> {
        return <Vec<Variant>>::from_variant(&self.imp().settings.clone().value("clocks"))
            .unwrap()
            .iter_mut()
            .map(|v| Location::world().unwrap().deserialize(v))
            .collect();
    }
}
