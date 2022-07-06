use gtk::{
    glib::{wrapper, Object},
    prelude::ToValue,
};

mod imp {
    use gtk::{
        glib::{
            self, object_subclass, once_cell::sync::Lazy, ParamFlags, ParamSpec, ParamSpecInt,
            ParamSpecUInt64,
        },
        prelude::ToValue,
        subclass::prelude::*,
    };
    use std::cell::Cell;

    #[derive(Default)]
    pub struct StopwatchLap {
        pub duration: Cell<u64>,
        pub index: Cell<u64>,
    }

    #[object_subclass]
    impl ObjectSubclass for StopwatchLap {
        const NAME: &'static str = "StopwatchLap";
        type Type = super::StopwatchLap;
    }

    impl ObjectImpl for StopwatchLap {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecUInt64::new(
                        "duration",
                        "",
                        "",
                        u64::MIN,
                        u64::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecInt::new(
                        "index",
                        "",
                        "",
                        i32::MIN,
                        i32::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(
            &self,
            _obj: &Self::Type,
            _id: usize,
            value: &glib::Value,
            pspec: &ParamSpec,
        ) {
            match pspec.name() {
                "duration" => self.duration.replace(
                    value
                        .get::<u64>()
                        .expect("Failed to get floating point value"),
                ),
                "index" => self.index.replace(
                    value
                        .get::<i32>()
                        .expect("Failed to get integer value")
                        .try_into()
                        .unwrap(),
                ),
                _ => unimplemented!(),
            };
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "duration" => self.duration.get().to_value(),
                "index" => (self.index.get() as i32).to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

wrapper! {
    pub struct StopwatchLap(ObjectSubclass<imp::StopwatchLap>);
}

impl Default for StopwatchLap {
    fn default() -> Self {
        Object::new(&[("duration", &(0 as u64).to_value()), ("index", &0.to_value())])
            .expect("Failed to create StopwatchLap")
    }
}

impl StopwatchLap {
    pub fn new(duration: u64, index: i32) -> Self {
        Object::new(&[
            ("duration", &duration.to_value()),
            ("index", &index.to_value()),
        ])
        .expect("Failed to create StopwatchLap")
    }
}
