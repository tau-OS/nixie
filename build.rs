use gio;
use owo_colors::OwoColorize;
use std::env;
use std::{
    io::{stderr, stdout, ErrorKind, Write},
    path::Path,
    process::Command,
};

fn get_icons(profile: String, base_id: &str) {
    let iconsdir = "/usr/share/icons";

    if profile == "debug" {
        execute_cmd(
            Command::new("sudo")
                .arg("cp")
                .arg(format!("data/icons/{}.svg", base_id.to_owned() + ".Devel"))
                .arg(format!("{}/hicolor/symbolic/apps", iconsdir)),
        );
    } else {
        execute_cmd(
            Command::new("sudo")
                .arg("cp")
                .arg(format!("data/icons/{}.svg", base_id))
                .arg(format!("{}/hicolor/symbolic/apps", iconsdir)),
        );
    }

    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg(format!("data/icons/{}-symbolic.svg", base_id))
            .arg(format!("{}/hicolor/symbolic/apps", iconsdir)),
    );
}

// Utility function to execute commands and print output
fn execute_cmd(cmd: &mut Command) {
    let cmd = cmd.output().expect("Failed to execute command");

    if !cmd.status.success() {
        stdout().write_all(&cmd.stdout).unwrap();
        stderr().write_all(&cmd.stderr).unwrap();
    }
}

// Utility function to execute commands, if they exist, and print output
fn execute_maybe_cmd(cmd: &mut Command) {
    match cmd.spawn() {
        Ok(_c) => {}
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                println!("Command {:?} not found", cmd.get_program())
            } else {
                println!("An unknown error occured");
            }
        }
    }
}

fn main() {
    let base_id = "co.tauos.Nixie";
    let out_dir = env::var("OUT_DIR").unwrap();
    let profile = env::var("PROFILE").unwrap();
    println!("cargo:rerun-if-changed=data");

    gio::compile_resources(
        "data",
        "data/co.tauos.Nixie.gresources.xml",
        "resources.gresource",
    );

    // Set Env Variables
    println!(
        "cargo:rustc-env=RESOURCES_FILE={}/resources.gresource",
        out_dir
    );

    if profile == "release" {
        println!("cargo:rustc-env=APP_PROFILE=release");
        println!("cargo:rustc-env=APP_SUFFIX=");
        println!("cargo:rustc-env=APP_ID={}", base_id);

        println!("cargo:rustc-env=RUST_LOG=info");
    } else if profile == "debug" {
        println!("cargo:rustc-env=APP_PROFILE=development");
        println!("cargo:rustc-env=APP_SUFFIX=-Devel");
        println!("cargo:rustc-env=APP_ID={}.Devel", base_id);

        println!("cargo:rustc-env=RUST_LOG=trace");
    }

    println!("Please enter your {} password", "sudo".red());
    println!("{}", "This is used to install schemas, metainfo".italic());

    // PERFORM VALIDATION
    /////////////////////

    execute_maybe_cmd(
        Command::new("appstream-util")
            .arg("validate")
            .arg("--nonet")
            .arg(Path::new("./data/co.tauos.Nixie.metainfo.xml")),
    );

    execute_maybe_cmd(
        Command::new("glib-compile-schemas")
            .arg("--strict")
            .arg("--dry-run")
            .arg("data/co.tauos.Nixie.gschema.xml"),
    );

    execute_maybe_cmd(Command::new("desktop-file-validate").arg("data/co.tauos.Nixie.desktop"));

    // INSTALL SCHEMAS
    //////////////////

    get_icons(profile, base_id);

    // TODO get /usr/share from environment
    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg("data/co.tauos.Nixie.gschema.xml")
            .arg("/usr/share/glib-2.0/schemas/"),
    );

    execute_cmd(
        Command::new("sudo")
            .arg("glib-compile-schemas")
            .arg("/usr/share/glib-2.0/schemas/"),
    );

    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg("data/co.tauos.Nixie.metainfo.xml")
            .arg("/usr/share/metainfo"),
    );

    execute_cmd(
        Command::new("sudo")
            .arg("cp")
            .arg("data/co.tauos.Nixie.desktop")
            .arg("/usr/share/applications"),
    );
}
