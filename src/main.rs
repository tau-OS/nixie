extern crate pretty_env_logger;
extern crate stopwatch;

mod alarm;
mod application;
mod clock;
mod clock_store;
mod config;
mod lap;
mod macros;
mod ui;
mod weekday;
mod window;

use config::RESOURCES_FILE;
use gettextrs::gettext;
use gtk::{gio, glib::set_application_name};

use self::application::Application;

fn main() {
    pretty_env_logger::init();

    // Prepare i18n
    // gettextrs::setlocale(LocaleCategory::LcAll, "");
    // gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    // gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    set_application_name(&gettext("Nixie"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let app = Application::new();
    app.run();
}
