/*
 * Copyright (c) 2022 Fyra Labs
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of the GNU General Public License
 * as published by the Free Software Foundation; either version 3
 * of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 */

[GtkTemplate (ui = "/com/fyralabs/Nixie/alarmface.ui")]
public class Nixie.AlarmFace : He.Bin {
    public MainWindow win { get; set; }

    private Utils.ContentStore alarms;
    private GLib.Settings settings;

    [GtkChild]
    private unowned Gtk.Stack stack;
    [GtkChild]
    private unowned He.EmptyPage emptypage;
    [GtkChild]
    private unowned Gtk.ListBox alarms_listbox;
    [GtkChild]
    private unowned Gtk.Box main_box;
    [GtkChild]
    private unowned He.AppBar empty_appbar;
    [GtkChild]
    private unowned He.AppBar alarms_appbar;
    [GtkChild]
    public unowned Gtk.MenuButton menu_button;
    [GtkChild]
    public unowned Gtk.MenuButton menu_button2;

    construct {
        emptypage.action_button.visible = false;

        alarms = new Utils.ContentStore ();
        settings = new GLib.Settings ("com.fyralabs.Nixie");

        alarms.set_sorting ((item1, item2) => {
            var alarm1 = (AlarmItem) item1;
            var alarm2 = (AlarmItem) item2;

            // Sort by time (hour then minute)
            if (alarm1.hour < alarm2.hour)
                return -1;
            if (alarm1.hour > alarm2.hour)
                return 1;
            if (alarm1.minute < alarm2.minute)
                return -1;
            if (alarm1.minute > alarm2.minute)
                return 1;
            return 0;
        });

        alarms_listbox.bind_model (alarms, (item) => {
            var row = new AlarmRow ((AlarmItem) item);
            row.remove_alarm.connect (() => remove_alarm ((AlarmItem) item));
            row.edit_alarm.connect (() => edit_alarm ((AlarmItem) item));
            return row;
        });

        load ();

        alarms.items_changed.connect ((position, removed, added) => {
            save ();
            update_view ();
        });

        // Set menu button popover arrow property
        menu_button.get_popover ().has_arrow = false;
        menu_button2.get_popover ().has_arrow = false;

        // Setup AppBar bindings after widget is realized
        realize.connect (() => {
            setup_appbar_bindings ();
        });
    }

    private void setup_appbar_bindings () {
        if (win == null) {
            return;
        }

        // Get the album from MainWindow through the overlay
        var about_overlay = win.about_overlay;
        if (about_overlay != null && about_overlay.child != null) {
            var album = about_overlay.child;

            // Connect to the folded property notify signal
            album.notify["folded"].connect (() => {
                bool folded;
                album.get ("folded", out folded);
                empty_appbar.show_left_title_buttons = folded;
                alarms_appbar.show_left_title_buttons = folded;
            });

            // Set initial state
            bool folded;
            album.get ("folded", out folded);
            empty_appbar.show_left_title_buttons = folded;
            alarms_appbar.show_left_title_buttons = folded;
        }
    }

    private void update_view () {
        var item_count = alarms.get_n_items ();
        message ("Updating view, alarm count: %u", item_count);

        if (item_count == 0) {
            message ("Switching to empty view");
            stack.set_visible_child_name ("empty");
        } else {
            message ("Switching to alarms view");
            stack.set_visible_child_name ("alarms");
        }
    }

    [GtkCallback]
    private void on_new_alarm () {
        activate_new ();
    }

    private void load () {
        try {
            alarms.deserialize (settings.get_value ("alarms"), AlarmItem.deserialize);
        } catch (Error e) {
            warning ("Failed to load alarms: %s", e.message);
            // Reset alarms setting if deserialization fails
            settings.reset ("alarms");
        }
        update_view ();
    }

    private void save () {
        try {
            settings.set_value ("alarms", alarms.serialize ());
        } catch (Error e) {
            warning ("Failed to save alarms: %s", e.message);
        }
    }

    private void remove_alarm (AlarmItem item) {
        alarms.remove (item);
    }

    private void add_alarm_item (AlarmItem item) {
        message ("Adding alarm item: %02d:%02d %s", item.hour, item.minute, item.name);
        alarms.add (item);
        message ("Alarm added, total items: %u", alarms.get_n_items ());
        save ();
    }

    private void edit_alarm (AlarmItem item) {
        var dialog = new AlarmSetupDialog ((Gtk.Window) get_root (), item);
        dialog.alarm_saved.connect ((updated_item) => {
            // Remove old item and add updated one
            alarms.remove (item);
            alarms.add (updated_item);
        });
        dialog.alarm_deleted.connect ((deleted_item) => {
            alarms.remove (deleted_item);
        });
        dialog.present ();
    }

    public void activate_new () {
        message ("Creating new alarm dialog");
        var dialog = new AlarmSetupDialog ((Gtk.Window) get_root (), null);
        dialog.alarm_saved.connect ((new_item) => {
            message ("Received alarm_saved signal");
            add_alarm_item (new_item);
        });
        dialog.alarm_deleted.connect ((deleted_item) => {
            // This shouldn't happen for new alarms, but handle it just in case
            alarms.remove (deleted_item);
        });
        dialog.present ();
    }
}

[GtkTemplate (ui = "/com/fyralabs/Nixie/alarmrow.ui")]
private class Nixie.AlarmRow : He.Bin {
    public AlarmItem alarm_item { get; construct set; }

    [GtkChild]
    private unowned Gtk.Label time_label;
    [GtkChild]
    private unowned Gtk.Label name_label;
    [GtkChild]
    private unowned Gtk.Label days_label;
    [GtkChild]
    private unowned He.Switch enabled_switch;

    internal signal void remove_alarm ();
    internal signal void edit_alarm ();

    public AlarmRow (AlarmItem alarm_item) {
        Object (alarm_item: alarm_item);

        alarm_item.notify.connect (update);
        enabled_switch.iswitch.notify["active"].connect (() => {
            alarm_item.enabled = enabled_switch.iswitch.active;
        });

        update ();
    }

    private void update () {
        if (alarm_item == null) {
            return;
        }

        time_label.label = alarm_item.time_label;
        name_label.label = alarm_item.name ?? _("Alarm");
        days_label.label = alarm_item.days_label;
        enabled_switch.iswitch.active = alarm_item.enabled;

        // Apply active/inactive styling
        if (alarm_item.enabled) {
            remove_css_class ("inactive");
        } else {
            add_css_class ("inactive");
        }
    }

    [GtkCallback]
    private void on_edit_clicked () {
        edit_alarm ();
    }

    [GtkCallback]
    private void on_delete_clicked () {
        remove_alarm ();
    }
}

public class Nixie.AlarmItem : Object, Nixie.Utils.ContentItem {
    public int hour { get; set; }
    public int minute { get; set; }
    private string _name = "";
    public bool enabled { get; set; default = true; }
    public Utils.Weekdays weekdays { get; set; }

    public string? name {
        get {
            return _name;
        }
        set {
            _name = value ?? "";
        }
    }

    public string time_label {
        owned get {
            var time = new GLib.DateTime.local (2000, 1, 1, hour, minute, 0);
            return Utils.WallClock.get_default ().format_time (time);
        }
    }

    public string days_label {
        owned get {
            return weekdays.get_label ();
        }
    }

    public AlarmItem (int hour = 0, int minute = 0, string name = "") {
        Object (hour: hour, minute: minute);

        _name = name;
        weekdays = new Utils.Weekdays ();
    }

    public void serialize (GLib.VariantBuilder builder) {
        try {
            builder.open (new GLib.VariantType ("a{sv}"));
            builder.add ("{sv}", "hour", new Variant.int32 (hour));
            builder.add ("{sv}", "minute", new Variant.int32 (minute));
            builder.add ("{sv}", "name", new Variant.string (_name ?? ""));
            builder.add ("{sv}", "enabled", new Variant.boolean (enabled));
            builder.add ("{sv}", "days", weekdays.serialize ());
            builder.close ();
        } catch (Error e) {
            warning ("Failed to serialize alarm item: %s", e.message);
        }
    }

    public static AlarmItem ? deserialize (Variant alarm_variant) {
        string key;
        Variant val;

        int hour = 0;
        int minute = 0;
        string name = "";
        bool enabled = true;
        Utils.Weekdays? weekdays = null;

        try {
            var iter = alarm_variant.iterator ();
            while (iter.next ("{sv}", out key, out val)) {
                switch (key) {
                case "hour" :
                    hour = val.get_int32 ();
                    break;
                case "minute" :
                    minute = val.get_int32 ();
                    break;
                case "name":
                    name = val.get_string ();
                    break;
                case "enabled":
                    enabled = val.get_boolean ();
                    break;
                case "days":
                    weekdays = Utils.Weekdays.deserialize (val);
                    break;
                }
            }

            // Validate data ranges
            if (hour < 0 || hour > 23 || minute < 0 || minute > 59) {
                warning ("Invalid alarm time: %02d:%02d", hour, minute);
                return null;
            }

            var item = new AlarmItem (hour, minute, name);
            item.enabled = enabled;
            if (weekdays != null) {
                item.weekdays = weekdays;
            }
            return item;
        } catch (Error e) {
            warning ("Failed to deserialize alarm item: %s", e.message);
            return null;
        }
    }
}