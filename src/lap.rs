use gtk::{
    glib::{wrapper, Object},
    prelude::ToValue,
};

mod imp {
    use gtk::{
        glib::{
            self, object_subclass, once_cell::sync::Lazy, ParamFlags, ParamSpec, ParamSpecDouble,
            ParamSpecInt,
        },
        prelude::ToValue,
        subclass::prelude::*,
    };
    use std::cell::Cell;

    #[derive(Default)]
    pub struct StopwatchLap {
        pub duration: Cell<f64>,
        pub index: Cell<f64>,
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
                    ParamSpecDouble::new(
                        "duration",
                        "",
                        "",
                        f64::MIN,
                        f64::MAX,
                        0.0,
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
                        .get::<f64>()
                        .expect("Failed to get floating point value"),
                ),
                "index" => self
                    .index
                    .replace(value.get::<i32>().expect("Failed to get integer value").into()),
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
        Object::new(&[("duration", &0.0.to_value()), ("index", &0.to_value())])
            .expect("Failed to create StopwatchLap")
    }
}

impl StopwatchLap {
    pub fn new(duration: f64, index: i32) -> Self {
        Object::new(&[
            ("duration", &duration.to_value()),
            ("index", &index.to_value()),
        ])
        .expect("Failed to create StopwatchLap")
    }
}
