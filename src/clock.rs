use gtk::{
    glib::{self, closure_local, wrapper, Object, SignalHandlerId},
    prelude::*,
};

mod imp {
    use std::time::Duration;

    use gtk::{
        glib::{
            self, clone, object_subclass, once_cell::sync::Lazy, subclass::Signal,
            timeout_add_local,
        },
        prelude::*,
    };
    use he::subclass::prelude::*;
    use log::debug;

    #[derive(Default)]
    pub struct Clock {}

    #[object_subclass]
    impl ObjectSubclass for Clock {
        const NAME: &'static str = "InternalClock";
        type Type = super::InternalClock;
    }

    impl ObjectImpl for Clock {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            debug!("GObject<InternalClock>::constructed");

            timeout_add_local(
                Duration::from_millis(1),
                clone!(@weak obj => @default-return Continue(false), move || {
                    obj.emit_by_name::<()>("tick", &[]);
                    Continue(true)
                }),
            );
        }

        fn signals() -> &'static [Signal] {
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder(
                    "tick",
                    &[],
                    <()>::static_type().into(),
                )
                .build()]
            });
            SIGNALS.as_ref()
        }
    }
}

wrapper! {
    pub struct InternalClock(ObjectSubclass<imp::Clock>);
}

impl Default for InternalClock {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalClock {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create InternalClock")
    }

    pub fn connect_tick<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        self.connect_closure(
            "tick",
            true,
            closure_local!(|ref clock| {
                f(clock);
            }),
        )
    }
}
