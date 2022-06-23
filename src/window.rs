pub mod window {
    use gtk::{Box, Orientation, Stack};
    use he::prelude::*;
    use he::{AppBar, ApplicationWindow, ViewSwitcher};

    use crate::{ui, Application};

    pub fn build(app: &Application) {
        let globalbox = Box::new(Orientation::Vertical, 0);

        let appbar = AppBar::builder()
            .show_buttons(true)
            .show_back(false)
            .build();

        let stack = Stack::new();

        let clocks = ui::clocks::ClocksPage::new();

        stack.add_titled(&clocks, None, "Clocks");
        stack.add_titled(&ui::alarms::alarms::build(), None, "Alarms");
        stack.add_titled(&ui::stopwatch::stopwatch::build(), None, "Stopwatch");
        stack.add_titled(&ui::timer::timer::build(), None, "Timer");

        stack.set_margin_start(12);
        stack.set_margin_end(12);

        let switcher = ViewSwitcher::builder()
            .stack(&stack)
            .margin_start(12)
            .margin_top(6)
            .build();

        globalbox.append(&appbar);
        globalbox.append(&switcher);
        globalbox.append(&stack);

        clocks.fill();

        let window = ApplicationWindow::builder()
            .application(app)
            .title("Nixie")
            .child(&globalbox)
            .build();

        window.present();
    }
}