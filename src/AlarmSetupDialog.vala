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

[GtkTemplate (ui = "/com/fyralabs/Nixie/alarmsetup.ui")]
public class Nixie.AlarmSetupDialog : He.Window {
    private AlarmItem? alarm_item;
    private bool is_editing;

    [GtkChild]
    private unowned Gtk.SpinButton h_spinbutton;
    [GtkChild]
    private unowned Gtk.SpinButton m_spinbutton;
    [GtkChild]
    private unowned He.TextField name_entry;
    [GtkChild]
    private unowned AlarmDayPickerRow repeats;
    [GtkChild]
    private unowned He.Button primary_button;
    [GtkChild]
    private unowned He.Button delete_button;
    [GtkChild]
    private unowned Gtk.Revealer label_revealer;

    public signal void alarm_saved (AlarmItem item);
    public signal void alarm_deleted (AlarmItem item);

    public AlarmSetupDialog (Gtk.Window parent, AlarmItem? item) {
        Object (transient_for : parent, modal : true);

        alarm_item = item;
        is_editing = (item != null);

        setup_ui ();
        load_alarm_data ();
    }

    construct {
        primary_button.clicked.connect (on_save);
        delete_button.clicked.connect (on_delete);

        name_entry.get_internal_entry ().changed.connect (entry_changed);
    }

    private void setup_ui () {
        if (is_editing) {
            title = _("Edit Alarm");
            primary_button.label = _("Save");
            delete_button.visible = true;
        } else {
            title = _("New Alarm");
            primary_button.label = _("Add");
            delete_button.visible = false;
        }
    }

    private void load_alarm_data () {
        if (is_editing && alarm_item != null) {
            h_spinbutton.value = alarm_item.hour;
            m_spinbutton.value = alarm_item.minute;
            name_entry.text = alarm_item.name;
            repeats.weekdays = alarm_item.weekdays;
        } else {
            // Set default values for new alarm
            var now = new GLib.DateTime.now_local ();
            h_spinbutton.value = now.get_hour ();
            m_spinbutton.value = now.get_minute ();
            name_entry.placeholder_text = _("Alarm");
            repeats.weekdays = new Utils.Weekdays ();
        }
    }

    [GtkCallback]
    private bool show_leading_zeros (Gtk.SpinButton spin_button) {
        spin_button.text = "%02d".printf ((int) spin_button.value);
        return true;
    }

    [GtkCallback]
    private void spinbuttons_changed () {
        validate_input ();
    }

    private void entry_changed () {
        validate_input ();
    }

    [GtkCallback]
    private void days_changed () {
        validate_input ();
    }

    private void validate_input () {
        // Always allow saving - alarms can be created without days selected
        primary_button.sensitive = true;
        label_revealer.reveal_child = false;

        // Could add duplicate checking here if needed
    }

    private void on_save () {
        int hour = (int) h_spinbutton.value;
        int minute = (int) m_spinbutton.value;
        string name = name_entry.text.strip ();

        if (name == "") {
            name = _("Alarm");
        }

        AlarmItem item;
        if (is_editing && alarm_item != null) {
            // Update existing item
            alarm_item.hour = hour;
            alarm_item.minute = minute;
            alarm_item.name = name;
            alarm_item.weekdays = repeats.weekdays;
            item = alarm_item;
        } else {
            // Create new item
            item = new AlarmItem (hour, minute, name);
            item.weekdays = repeats.weekdays;
        }

        alarm_saved (item);
        destroy ();
    }

    [GtkCallback]
    private void on_delete () {
        if (is_editing && alarm_item != null) {
            alarm_deleted (alarm_item);
        }
        destroy ();
    }
}

public class Nixie.AlarmDayPickerRow : Gtk.Box {
    private He.Button[] day_buttons;
    private Utils.Weekdays _weekdays;

    public Utils.Weekdays weekdays {
        get { return _weekdays; }
        set {
            _weekdays = value;
            update_buttons ();
        }
    }

    public signal void days_changed ();

    construct {
        orientation = Gtk.Orientation.HORIZONTAL;
        halign = Gtk.Align.CENTER;
        add_css_class ("grouped-button");
        spacing = 4;
        margin_top = 12;

        // Initialize weekdays
        _weekdays = new Utils.Weekdays ();
        day_buttons = new He.Button[7];

        // Create buttons for each day, starting with the first weekday
        var first_weekday = Utils.Weekdays.Day.get_first_weekday ();

        for (int i = 0; i < 7; i++) {
            var day = (Utils.Weekdays.Day) ((first_weekday + i) % 7);
            var button = new He.Button ("", day.symbol ());
            button.tooltip_text = day.name ();
            button.is_fill = true;
            button.toggle_mode = true;
            button.width_request = 45;
            button.height_request = 40;

            // Capture day for closure
            Utils.Weekdays.Day button_day = day;
            button.toggled.connect (() => {
                _weekdays.set (button_day, button.active);
                days_changed ();
            });

            day_buttons[i] = button;
            append (button);
        }
    }

    private void update_buttons () {
        if (_weekdays == null) {
            return;
        }

        var first_weekday = Utils.Weekdays.Day.get_first_weekday ();

        for (int i = 0; i < 7; i++) {
            var day = (Utils.Weekdays.Day) ((first_weekday + i) % 7);
            day_buttons[i].active = _weekdays.get (day);
        }
    }
}