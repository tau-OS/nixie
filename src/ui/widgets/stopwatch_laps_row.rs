use chrono::Duration;
use gtk::{
    glib::{self, Object},
    prelude::*,
    subclass::prelude::*,
    Accessible, Actionable, Buildable, ConstraintTarget, ListBoxRow, Widget,
};

use crate::lap::StopwatchLap;

mod imp {
    use gtk::{
        glib::{self, subclass::InitializingObject},
        subclass::prelude::*,
        subclass::{list_box_row::ListBoxRowImpl, prelude::WidgetImpl},
        CompositeTemplate, Label, ListBoxRow,
    };
    use he::prelude::*;
    use log::debug;

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/co/tauos/Nixie/stopwatch_laps.ui")]
    pub struct StopwatchLapsRow {
        #[template_child]
        pub index_label: TemplateChild<Label>,
        #[template_child]
        pub difference_label: TemplateChild<Label>,
        #[template_child]
        pub duration_label: TemplateChild<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StopwatchLapsRow {
        const NAME: &'static str = "NixieStopwatchLapsRow";
        type Type = super::StopwatchLapsRow;
        type ParentType = ListBoxRow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ListBoxRowImpl for StopwatchLapsRow {}
    impl ObjectImpl for StopwatchLapsRow {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            obj.connect_realize(move |_| {
                debug!("GtkListBoxRow<StopwatchLapsRow>::realize");
            });
        }
    }
    impl WidgetImpl for StopwatchLapsRow {}
}

glib::wrapper! {
    pub struct StopwatchLapsRow(ObjectSubclass<imp::StopwatchLapsRow>)
        @extends ListBoxRow, Widget,
        @implements Accessible, Buildable, ConstraintTarget, Actionable;
}

impl StopwatchLapsRow {
    pub fn new(current: StopwatchLap, before: Option<StopwatchLap>) -> Self {
        let obj: StopwatchLapsRow = Object::new(&[]).expect("Failed to create StopwatchLapsRow");

        obj.imp().index_label.set_label(
            format!(
                "Lap {}",
                current.property_value("index").get::<i32>().unwrap()
            )
            .as_ref(),
        );

        obj.imp().duration_label.set_label(&Self::get_duration(
            current.property_value("duration").get::<f64>().unwrap(),
        ));

        if before != None {
            // TODO delta label
            obj.imp().difference_label.set_label (&Self::get_delta_label(current.clone(), before.clone()));

            let diff = Self::get_delta_duration(current, before);
            if diff > 0.0 {
                // Add CSS class
                obj.add_css_class("error");
            } else if diff < 0.0 {
                obj.add_css_class("accent");
            }
        }

        obj
    }

    fn get_delta_label(current: StopwatchLap, before: Option<StopwatchLap>) -> String {
        if before != None {
            let diff = current.property_value("duration").get::<f64>().unwrap();
            let label = Self::get_duration(diff);
            if diff < 0.0 {
                return format!("-{}", label);
            } else {
                return format!("+{}", label);
            }
        }
        return "".to_string();
    }

    fn get_delta_duration(current: StopwatchLap, before: Option<StopwatchLap>) -> f64 {
        if before != None {
            return current.property_value("duration").get::<f64>().unwrap()
                - before
                    .unwrap()
                    .property_value("duration")
                    .get::<f64>()
                    .unwrap();
        }
        return 0.0;
    }

    fn get_duration(duration: f64) -> String {
        // math!
        let time = Duration::seconds(((duration * 100.0).floor() / 100.0) as i64);
        let ms = ((time.num_milliseconds() / 100) % 10) * 10;

        return format!(
            "{}\u{200E} ∶{}\u{200E} ∶{}.{}",
            time.num_hours().abs(),
            time.num_minutes().abs(),
            time.num_seconds().abs(),
            ms.abs()
        );
    }
}
