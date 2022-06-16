use he::prelude::*;
use he::{Application};

mod ui;
mod window;
use crate::window::window::build;

fn main() {
    let app = Application::builder()
        .application_id("co.tauos.Nixie")
        .build();

    app.connect_activate(build);

    app.run();
}
