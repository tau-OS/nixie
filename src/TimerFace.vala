/*
 * Copyright (C) 2013  Paolo Borelli <pborelli@gnome.org>
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

public class Nixie.TimerItem : Object, Nixie.Utils.ContentItem {
    public enum State {
        STOPPED,
        RUNNING,
        PAUSED
    }

    public State state { get; private set; default = State.STOPPED; }

    public string? name { get; set; }
    public int hours { get; set; default = 0; }
    public int minutes { get; set; default = 0; }
    public int seconds { get; set; default = 0; }

    private double span;
    private GLib.Timer timer;
    private uint timeout_id;
    private int stored_hour;
    private int stored_minute;
    private int stored_second;


    public signal void ring ();
    public signal void countdown_updated (int hours, int minutes, int seconds);

    public int get_total_seconds () {
        return hours * 3600 + minutes * 60 + seconds;
    }

    public void serialize (GLib.VariantBuilder builder) {
        builder.open (new GLib.VariantType ("a{sv}"));
        builder.add ("{sv}", "duration", new GLib.Variant.int32 (get_total_seconds ()));
        if (name != null) {
            builder.add ("{sv}", "name", new GLib.Variant.string ((string) name));
        }
        builder.close ();
    }

    public static TimerItem ? deserialize (Variant time_variant) {
        string key;
        Variant val;
        int duration = 0;
        string? name = null;

        var iter = time_variant.iterator ();
        while (iter.next ("{sv}", out key, out val)) {
            switch (key) {
            case "duration" :
                duration = (int32) val;
                break;
            case "name":
                name = (string) val;
                break;
            }
        }

        return duration != 0 ? (TimerItem?) new TimerItem.from_seconds (duration, name) : null;
    }

    public TimerItem.from_seconds (int seconds, string? name) {

        int rest = 0;
        int h = seconds / 3600;
        rest = seconds - h * 3600;
        int m = rest / 60;
        int s = rest - m * 60;

        this(h, m, s, name);
    }

    public TimerItem (int h, int m, int s, string? name) {
        Object (name : name);
        hours = h;
        minutes = m;
        seconds = s;

        span = get_total_seconds ();
        timer = new GLib.Timer ();

        timeout_id = 0;
    }

    public virtual signal void start () {
        state = State.RUNNING;
        timeout_id = GLib.Timeout.add (100, () => {
            var e = timer.elapsed ();
            if (state != State.RUNNING) {
                return false;
            }
            if (e >= span) {
                reset ();
                ring ();
                timeout_id = 0;
                return false;
            }
            var elapsed = Math.ceil (span - e);
            int h;
            int m;
            int s;
            double r;
            Utils.Misc.time_to_hms (elapsed, out h, out m, out s, out r);

            if (stored_hour != h || stored_minute != m || stored_second != s) {
                stored_hour = h;
                stored_minute = m;
                stored_second = s;
                countdown_updated (h, m, s);
            }
            return true;
        });
        timer.start ();
    }

    public virtual signal void pause () {
        state = State.PAUSED;
        span -= timer.elapsed ();
        timer.stop ();
    }

    public virtual signal void reset () {
        state = State.STOPPED;
        span = get_total_seconds ();
        timer.reset ();
        timeout_id = 0;
    }
}

[GtkTemplate (ui = "/com/fyralabs/Nixie/timerrow.ui")]
public class Nixie.TimerRow : He.Bin {
    public Nixie.TimerItem item {
        get {
            return _item;
        }

        construct set {
            _item = value;

            title.text = (string) _item.name;
            title.get_internal_entry ().bind_property ("text", _item, "name");
            timer_name.label = (string) _item.name;
            title.get_internal_entry ().bind_property ("text", timer_name, "label");

            _item.notify["name"].connect (() => edited ());
        }
    }
    private Nixie.TimerItem _item;

    [GtkChild]
    private unowned Gtk.Label countdown_label;

    [GtkChild]
    private unowned Gtk.Label timer_name;

    [GtkChild]
    private unowned Gtk.Stack name_stack;
    [GtkChild]
    private unowned Gtk.Revealer name_revealer;

    [GtkChild]
    private unowned Gtk.Stack start_stack;
    [GtkChild]
    private unowned Gtk.Stack reset_stack;
    [GtkChild]
    private unowned Gtk.Stack delete_stack;

    [GtkChild]
    private unowned Gtk.Button delete_button;
    [GtkChild]
    private unowned He.TextField title;

    public signal void deleted ();
    public signal void edited ();

    public TimerRow (Nixie.TimerItem item) {
        Object (item: item);
        countdown_label.set_direction (Gtk.TextDirection.LTR);

        title.get_internal_entry ().width_chars = 26;

        item.countdown_updated.connect (this.update_countdown);
        item.ring.connect (() => this.ring ());
        item.start.connect (() => this.start ());
        item.pause.connect (() => this.pause ());
        item.reset.connect (() => this.reset ());
        delete_button.clicked.connect (() => deleted ());

        reset ();
    }

    [GtkCallback]
    private void on_start_button_clicked () {
        item.start ();
    }

    [GtkCallback]
    private void on_pause_button_clicked () {
        item.pause ();
    }

    [GtkCallback]
    private void on_reset_button_clicked () {
        item.reset ();
    }

    private void reset () {
        reset_stack.visible_child_name = "empty";
        delete_stack.visible_child_name = "button";

        countdown_label.remove_css_class ("accent");
        countdown_label.add_css_class ("dim-label");

        start_stack.visible_child_name = "start";
        name_revealer.reveal_child = true;
        name_stack.visible_child_name = "edit";

        update_countdown (item.hours, item.minutes, item.seconds);
    }

    private void start () {
        countdown_label.add_css_class ("accent");
        countdown_label.remove_css_class ("dim-label");

        reset_stack.visible_child_name = "empty";
        delete_stack.visible_child_name = "empty";

        start_stack.visible_child_name = "pause";
        name_revealer.reveal_child = (timer_name.label != "");
        name_stack.visible_child_name = "display";
    }

    private void ring () {
        countdown_label.remove_css_class ("accent");
        countdown_label.add_css_class ("dim-label");
    }

    private void pause () {
        reset_stack.visible_child_name = "button";
        delete_stack.visible_child_name = "button";
        start_stack.visible_child_name = "start";
        name_revealer.reveal_child = (timer_name.label != "");
        name_stack.visible_child_name = "display";
    }

    private void update_countdown (int h, int m, int s) {
        countdown_label.set_text ("%02i ∶ %02i ∶ %02i".printf (h, m, s));
    }
}

[GtkTemplate (ui = "/com/fyralabs/Nixie/timerface.ui")]
public class Nixie.TimerFace : He.Bin, Nixie.Utils.Clock {
    public MainWindow win { get; set; }

    [GtkChild]
    private unowned Gtk.ListBox timers_list;
    [GtkChild]
    private unowned He.Button start_button;
    [GtkChild]
    private unowned Gtk.Stack stack;
    [GtkChild]
    private unowned Gtk.Box main_box;
    [GtkChild]
    public unowned Gtk.MenuButton menu_button;
    [GtkChild]
    private unowned He.AppBar timer_appbar;

    // Timer setup widgets
    [GtkChild]
    private unowned Gtk.SpinButton hours_spin;
    [GtkChild]
    private unowned Gtk.SpinButton minutes_spin;
    [GtkChild]
    private unowned Gtk.SpinButton seconds_spin;
    [GtkChild]
    private unowned He.TextField timer_name_entry;
    [GtkChild]
    private unowned He.Button preset_5min;
    [GtkChild]
    private unowned He.Button preset_10min;
    [GtkChild]
    private unowned He.Button preset_30min;
    [GtkChild]
    private unowned He.Button preset_45min;

    public bool is_running { get; set; default = false; }

    private Utils.ContentStore timers;
    private GLib.Settings settings;
    private Utils.Bell bell;
    private GLib.Notification notification;

    construct {
        settings = new GLib.Settings ("com.fyralabs.Nixie");
        timers = new Utils.ContentStore ();

        timers_list.bind_model (timers, (timer) => {
            var row = new TimerRow ((TimerItem) timer);
            row.deleted.connect (() => remove_timer ((TimerItem) timer));
            row.edited.connect (() => save ());
            ((TimerItem) timer).ring.connect (() => ring ());
            ((TimerItem) timer).notify["state"].connect (() => {
                this.is_running = this.get_total_active_timers () != 0;
            });
            return row;
        });

        timers.items_changed.connect ((added, removed, position) => {
            if (this.timers.get_n_items () > 0) {
                stack.visible_child_name = "timers";
            } else {
                stack.visible_child_name = "setup";
            }
            save ();
        });

        bell = new Utils.Bell ("complete");
        notification = new GLib.Notification (_("Time is up!"));
        notification.set_body (_("Timer countdown finished"));
        notification.set_priority (HIGH);

        stack.set_visible_child_name ("setup");
        start_button.set_sensitive (false);

        load ();
        menu_button.get_popover ().has_arrow = false;

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
                timer_appbar.show_left_title_buttons = folded;
            });

            // Set initial state
            bool folded;
            album.get ("folded", out folded);
            timer_appbar.show_left_title_buttons = folded;
        }
    }

    [GtkCallback]
    private void on_duration_changed () {
        int total_seconds = get_current_duration ();
        start_button.set_sensitive (total_seconds > 0);
    }

    [GtkCallback]
    private void on_preset_clicked (Gtk.Button button) {
        if (button == preset_5min) {
            set_timer_duration (0, 5, 0);
        } else if (button == preset_10min) {
            set_timer_duration (0, 10, 0);
        } else if (button == preset_30min) {
            set_timer_duration (0, 30, 0);
        } else if (button == preset_45min) {
            set_timer_duration (0, 45, 0);
        }
    }

    [GtkCallback]
    private void on_start_timer () {
        int hours = (int) hours_spin.value;
        int minutes = (int) minutes_spin.value;
        int seconds = (int) seconds_spin.value;
        string name = timer_name_entry.get_internal_entry ().text.strip ();

        if (name == "") {
            name = _("Timer");
        }

        var timer = new TimerItem (hours, minutes, seconds, name);
        timers.add (timer);
        timer.start ();

        // Reset the setup form
        reset_timer_setup ();
    }

    private void set_timer_duration (int hours, int minutes, int seconds) {
        hours_spin.value = hours;
        minutes_spin.value = minutes;
        seconds_spin.value = seconds;
        on_duration_changed ();
    }

    private int get_current_duration () {
        return (int) hours_spin.value * 3600 + (int) minutes_spin.value * 60 + (int) seconds_spin.value;
    }

    private void reset_timer_setup () {
        hours_spin.value = 0;
        minutes_spin.value = 0;
        seconds_spin.value = 0;
        timer_name_entry.text = "";
        start_button.set_sensitive (false);
    }

    private int get_total_active_timers () {
        var total_items = 0;
        this.timers.foreach ((timer) => {
            if (((TimerItem) timer).state == TimerItem.State.RUNNING) {
                total_items += 1;
            }
        });
        return total_items;
    }

    private void remove_timer (TimerItem item) {
        timers.remove (item);
    }

    private void load () {
        try {
            timers.deserialize (settings.get_value ("timers"), TimerItem.deserialize);
        } catch (Error e) {
            warning ("Failed to load timers: %s", e.message);
            settings.reset ("timers");
        }
    }

    private void save () {
        try {
            settings.set_value ("timers", timers.serialize ());
        } catch (Error e) {
            warning ("Failed to save timers: %s", e.message);
        }
    }

    public virtual signal void ring () {
        var app = (Nixie.Application) GLib.Application.get_default ();
        app.send_notification ("timer-is-up", notification);
        bell.ring_once ();
    }

    public override bool grab_focus () {
        if (timers.get_n_items () == 0) {
            start_button.grab_focus ();
            return true;
        }

        return false;
    }

    public bool escape_pressed () {
        var res = false;
        this.timers.foreach ((item) => {
            var timer = (TimerItem) item;
            if (timer.state == TimerItem.State.RUNNING) {
                timer.pause ();
                res = true;
            }
        });
        return res;
    }
}