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
*/
[GtkTemplate (ui = "/co/tauos/Nixie/mainwindow.ui")]
public class Nixie.MainWindow : He.ApplicationWindow {
    public He.Application app {get; construct;}
    public SimpleActionGroup actions { get; construct; }
    public const string ACTION_PREFIX = "win.";
    public const string ACTION_ABOUT = "action_about";

    [GtkChild]
    public unowned Gtk.Stack stack;
    [GtkChild]
    private unowned He.NavigationRail folded_rail;

    private const GLib.ActionEntry[] ACTION_ENTRIES = {
          {ACTION_ABOUT, action_about }
    };
    public static Gee.MultiMap<string, string> action_accelerators = new Gee.HashMultiMap<string, string> ();

    public MainWindow (He.Application application) {
        Object (
            app: application,
            application: application,
            icon_name: Config.APP_ID,
            title: _("Nixie")
        );
    }

    construct {
        // Actions
        actions = new SimpleActionGroup ();
        actions.add_action_entries (ACTION_ENTRIES, this);
        insert_action_group ("win", actions);

        foreach (var action in action_accelerators.get_keys ()) {
            var accels_array = action_accelerators[action].to_array ();
            accels_array += null;

            app.set_accels_for_action (ACTION_PREFIX + action, accels_array);
        }
        app.set_accels_for_action("app.quit", {"<Ctrl>q"});
        app.set_accels_for_action ("win.action_keys", {"<Ctrl>question"});

        var theme = Gtk.IconTheme.get_for_display (Gdk.Display.get_default ());
        theme.add_resource_path ("/co/tauos/Nixie/");

        set_size_request (360, 360);
        stack.visible_child_name = "clocks";

        ((Gtk.BoxLayout) folded_rail.get_layout_manager ()).orientation = Gtk.Orientation.HORIZONTAL;
        folded_rail.halign = Gtk.Align.CENTER;
    }

    public void action_about () {
        var about = new He.AboutWindow (
            this,
            "Nixie",
            Config.APP_ID,
            Config.VERSION,
            Config.APP_ID,
            "https://github.com/tau-os/nixie/tree/main/po",
            "https://github.com/tau-os/nixie/issues/new",
            "https://github.com/tau-os/nixie",
            // TRANSLATORS: 'Name <email@domain.com>' or 'Name https://website.example'
            {},
            {"Lains"},
            2022,
            He.AboutWindow.Licenses.GPLv3,
            He.Colors.INDIGO
        );
        about.present ();
    }
}
