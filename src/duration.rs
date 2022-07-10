use gtk::{
    gio::ListStore,
    glib::{wrapper, Object, Type},
    prelude::*,
    Label, SignalListItemFactory, Widget,
};

mod imp {
    use gtk::{
        glib::{
            self, object_subclass, once_cell::sync::Lazy, ParamFlags, ParamSpec, ParamSpecInt,
            ParamSpecString,
        },
        prelude::*,
        subclass::prelude::*,
    };
    use std::cell::{Cell, RefCell};

    #[derive(Default)]
    pub struct Duration {
        pub minutes: Cell<i32>,
        pub label: RefCell<String>,
    }

    #[object_subclass]
    impl ObjectSubclass for Duration {
        const NAME: &'static str = "Duration";
        type Type = super::Duration;
    }

    impl ObjectImpl for Duration {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecInt::new(
                        "minutes",
                        "",
                        "",
                        i32::MIN,
                        i32::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new("label", "", "", None, ParamFlags::READWRITE),
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
            if pspec.name() == "minutes" {
                self.minutes
                    .replace(value.get::<i32>().expect("Failed to get integer value"));
            } else if pspec.name() == "label" {
                self.label
                    .replace(value.get::<String>().expect("Failed to get string value"));
            } else {
                unimplemented!()
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> glib::Value {
            match pspec.name() {
                "minutes" => self.minutes.get().to_value(),
                "label" => (self.label.borrow()).to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

wrapper! {
    pub struct Duration(ObjectSubclass<imp::Duration>);
}

impl Default for Duration {
    fn default() -> Self {
        Object::new(&[
            ("minutes", &0.to_value()),
            ("label", &String::default().to_value()),
        ])
        .expect("Failed to create Duration")
    }
}

impl Duration {
    pub fn new(minutes: i32, label: &str) -> Self {
        Object::new(&[
            ("minutes", &minutes.to_value()),
            ("label", &label.to_string().to_value()),
        ])
        .expect("Failed to create Duration")
    }

    pub fn model() -> ListStore {
        let store = ListStore::new(Type::OBJECT);

        store.append(&Duration::new(1, "1 Minute"));
        store.append(&Duration::new(5, "5 Minutes"));
        store.append(&Duration::new(10, "10 Minutes"));
        store.append(&Duration::new(15, "15 Minutes"));
        store.append(&Duration::new(20, "20 Minutes"));
        store.append(&Duration::new(30, "30 Minutes"));

        store
    }

    pub fn factory() -> SignalListItemFactory {
        let factory = SignalListItemFactory::new();
        factory.connect_setup(move |_, list_item| {
            let label = Label::new(None);
            list_item.set_child(Some(&label));
            list_item
                .property_expression("item")
                .chain_property::<Duration>("label")
                .bind(&label, "label", Widget::NONE);
        });
        factory
    }
}
