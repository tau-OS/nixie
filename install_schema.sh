# TODO make this a build.rs file but it's 3am

sudo cp data/co.tauos.Nixie.gschema.xml /usr/share/glib-2.0/schemas/
sudo glib-compile-schemas /usr/share/glib-2.0/schemas/
