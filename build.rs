use gio;

fn main() {
    gio::compile_resources(
        "data",
        "data/co.tauos.Nixie.gresources.xml",
        "co.tauos.Nixie.gresource",
    );
}