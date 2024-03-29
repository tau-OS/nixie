<img align="left" style="vertical-align: middle" width="120" height="120" src="data/icons/128/com.fyralabs.Nixie.svg">

# Nixie

Get the Time

![Screenshot](data/shot.png)

###

[![Please do not theme this app](https://stopthemingmy.app/badge.svg)](https://stopthemingmy.app)
[![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](http://www.gnu.org/licenses/gpl-3.0)

## 🛠️ Dependencies

You'll need the following dependencies:

> _Note_: This dependency list is the names searched for by `pkg-config`. Depending on your distribution, you may need to install other packages (for example, `gtk4-devel` on Fedora)

- `valac`
- `gtk4`
- `libhelium-1`

## 🏗️ Building

Simply clone this repo, then run `meson build` to configure the build environment. Change to the build directory and run `ninja test` to build and run automated tests.

```bash
$ meson build --prefix=/usr
$ cd build
$ ninja test
```

For debug messages on the GUI application, set the `G_MESSAGES_DEBUG` environment variable, e.g. to `all`:

```bash
G_MESSAGES_DEBUG=all ./src/com.fyralabs.Nixie
```

## 📦 Installing

To install, use `ninja install`, then execute with `com.fyralabs.Nixie`.

```bash
$ sudo ninja install
$ com.fyralabs.Nixie
```