/*
* Copyright (c) 2022 Fyra Labs
*
* This program is free software; you can redistribute it and/or
* modify it under the terms of the GNU General Public
* License as published by the Free Software Foundation; either
* version 3 of the License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
* General Public License for more details.
*
* You should have received a copy of the GNU General Public
* License along with this program; if not, write to the
* Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
* Boston, MA 02110-1301 USA
*
*/

public class Nixie.Application : He.Application {
    private const GLib.ActionEntry app_entries[] = {
        { "quit", quit },
    };

    public Application () {
        Object (application_id: Config.APP_ID);
    }
    public static int main (string[] args) {
        Intl.bindtextdomain (Config.GETTEXT_PACKAGE, Config.LOCALEDIR);
        Intl.textdomain (Config.GETTEXT_PACKAGE);
        Intl.bind_textdomain_codeset (Config.GETTEXT_PACKAGE, "UTF-8");

        var app = new Nixie.Application ();
        return app.run (args);
    }
    protected override void startup () {
        Gdk.RGBA accent_color = { 0 };
        accent_color.parse("#4426fa");
        default_accent_color = He.from_gdk_rgba(accent_color);

        resource_base_path = "/com/fyralabs/Nixie";

        base.startup ();

        Bis.init ();

        add_action_entries (app_entries, this);

        new MainWindow (this);
    }
    protected override void activate () {
        active_window?.present ();
    }
}


