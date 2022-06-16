use he::prelude::*;
use he::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
	.application_id("co.tauos.Nixie")
	.build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
	.application(app)
	.title("Nixie")
	.build();

    window.present();
}
