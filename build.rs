use gio;
use std::{
    io::{stderr, stdout, Write},
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=data");

    gio::compile_resources(
        "data",
        "data/co.tauos.Nixie.gresources.xml",
        "co.tauos.Nixie.gresource",
    );

    // TODO: See if there's a way to do this that won't rely on Command
    let cpout = Command::new("sudo")
        .arg("cp")
        .arg("data/co.tauos.Nixie.gschema.xml")
        .arg("/usr/share/glib-2.0/schemas/")
        .output()
        .expect("Failed to copy Schema");

    let comout = Command::new("sudo")
        .arg("glib-compile-schemas")
        .arg("/usr/share/glib-2.0/schemas/")
        .output()
        .expect("Failed to compile Schemas");

    if !cpout.status.success() {
        stdout().write_all(&cpout.stdout).unwrap();
        stderr().write_all(&cpout.stderr).unwrap();
    }

    if !comout.status.success() {
        stdout().write_all(&comout.stdout).unwrap();
        stderr().write_all(&comout.stderr).unwrap();
    }
}
