#[derive(Clone, Copy, PartialEq, Eq)]
pub enum State {
    Stopped,
    Reset,
    Running,
}
impl Default for State {
    fn default() -> Self {
        Self::Stopped
    }
}

mod imp {
    use chrono::Duration;
    use gtk::{
        glib::{self, subclass::InitializingObject, timeout_add_local, clone},
        prelude::*,
        subclass::prelude::*,
        template_callbacks, Box, Button, CompositeTemplate, Label,
    };
    use he::{traits::ButtonExt as HeButtonExt, Colors, FillButton};
    use std::sync::mpsc;
    use std::{
        cell::Cell,
        sync::mpsc::{Receiver, Sender},
    };
    use stopwatch::Stopwatch;

    use super::State;

    #[derive(CompositeTemplate)]
    #[template(resource = "/co/tauos/Nixie/stopwatch.ui")]
    pub struct StopwatchPage {
        #[template_child]
        pub time_container: TemplateChild<Box>,

        #[template_child]
        pub hours_label: TemplateChild<Label>,
        #[template_child]
        pub minutes_label: TemplateChild<Label>,
        #[template_child]
        pub seconds_label: TemplateChild<Label>,
        #[template_child]
        pub miliseconds_label: TemplateChild<Label>,

        #[template_child]
        pub start_btn: TemplateChild<FillButton>,
        #[template_child]
        pub clear_btn: TemplateChild<FillButton>,

        pub timer: Cell<Stopwatch>,
        pub state: Cell<State>,

        pub time_thread: (Sender<()>, Receiver<()>),
    }

    impl Default for StopwatchPage {
        fn default() -> Self {
            Self {
                time_container: TemplateChild::default(),
                hours_label: TemplateChild::default(),
                minutes_label: TemplateChild::default(),
                seconds_label: TemplateChild::default(),
                miliseconds_label: TemplateChild::default(),
                start_btn: TemplateChild::default(),
                clear_btn: TemplateChild::default(),
                timer: Cell::new(Stopwatch::new()),
                state: Cell::new(State::Stopped),
                time_thread: mpsc::channel(),
            }
        }
    }

    #[template_callbacks]
    impl StopwatchPage {
        fn start(&self) {
            self.timer.replace(Stopwatch::start_new());
            self.state.replace(State::Running);

            self.start_btn.set_label("Pause");
            self.start_btn.set_color(Colors::Yellow);

            self.time_container.add_css_class("running-stopwatch");
            self.time_container.remove_css_class("paused-stopwatch");
            self.time_container.remove_css_class("stopped-stopwatch");
        }

        pub fn update_time(&self) {
            let duration = Duration::from_std(self.timer.get().elapsed()).unwrap();

            let ms = (duration.num_milliseconds() / 100) % 10;

            self.hours_label
                .set_label(&format!("{}\u{200E}", duration.num_hours()));
            self.minutes_label
                .set_label(&format!("{}\u{200E}", duration.num_minutes()));
            self.seconds_label
                .set_label(&format!("{}\u{200E}", duration.num_seconds()));
            self.miliseconds_label
                .set_label(&format!("{}", ms));
        }

        #[template_callback]
        fn handle_on_start_btn_click(&self, _button: &Button) {
            match self.state.get() {
                State::Stopped => self::StopwatchPage::start(self),
                State::Running => todo!(),
                _ => unimplemented!(),
            }
        }
    }

    #[glib::object_subclass]
    impl ObjectSubclass for StopwatchPage {
        const NAME: &'static str = "NixieStopwatchPage";
        type Type = super::StopwatchPage;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl BoxImpl for StopwatchPage {}
    impl ObjectImpl for StopwatchPage {
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);

            self.timer.replace(Stopwatch::new());

            timeout_add_local(
                std::time::Duration::from_millis(1),
                clone!(@weak obj => @default-return Continue(false), move || {
                    if obj.imp().state.get() == State::Running {
                        obj.imp().update_time();
                    }
                    Continue(true)
                }),
            );
        }
    }

    impl WidgetImpl for StopwatchPage {}
}

use gtk::{
    glib::{self, Object},
    Accessible, Box, Buildable, ConstraintTarget, Widget,
};

glib::wrapper! {
    pub struct StopwatchPage(ObjectSubclass<imp::StopwatchPage>)
        @extends Box, Widget,
        @implements Accessible, Buildable, ConstraintTarget;
}

impl StopwatchPage {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create StopwatchPage")
    }
}
