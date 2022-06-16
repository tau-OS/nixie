pub mod window {
    use gtk::{Box, Orientation, Stack};
    use he::prelude::*;
    use he::{AppBar, Application, ApplicationWindow, ViewSwitcher};

    use crate::ui;

    pub fn build(app: &Application) {
        let globalbox = Box::new(Orientation::Vertical, 0);

        let appbar = AppBar::builder()
            .show_buttons(true)
            .show_back(false)
            .build();

        let stack = Stack::new();

        stack.add_titled(&ui::clocks::clocks::build(), None, "Clocks");
        stack.add_titled(&ui::alarms::alarms::build(), None, "Alarms");
        stack.add_titled(&ui::stopwatch::stopwatch::build(), None, "Stopwatch");
        stack.add_titled(&ui::timer::timer::build(), None, "Timer");

        let switcher = ViewSwitcher::builder()
            .stack(&stack)
            .margin_start(6)
            .margin_top(6)
            .build();

        globalbox.append(&appbar);
        globalbox.append(&switcher);
        globalbox.append(&stack);

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Nixie")
            .child(&globalbox)
            .build();

        window.present();
    }
}
