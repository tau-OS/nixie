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

        message ("Creating AlarmSetupDialog");

        alarm_item = item;
        is_editing = (item != null);

        setup_ui ();
        load_alarm_data ();

        message ("AlarmSetupDialog created and initialized");
    }

    construct {
        name_entry.get_internal_entry ().changed.connect (entry_changed);
        repeats.days_changed.connect (days_changed);
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

        message ("Dialog setup - Title: %s, Button: %s", title, primary_button.label);
    }

    private void load_alarm_data () {
        if (is_editing && alarm_item != null) {
            h_spinbutton.value = alarm_item.hour;
            m_spinbutton.value = alarm_item.minute;
            var alarm_name = alarm_item.name ?? _("Alarm");
            name_entry.text = alarm_name;
            repeats.weekdays = alarm_item.weekdays;
            message ("Loaded alarm data: %02d:%02d '%s'", alarm_item.hour, alarm_item.minute, alarm_name);
        } else {
            // Set default values for new alarm
            var now = new GLib.DateTime.now_local ();
            h_spinbutton.value = now.get_hour ();
            m_spinbutton.value = now.get_minute ();
            name_entry.placeholder_text = _("Alarm");
            name_entry.text = ""; // Set empty text instead of just placeholder
            repeats.weekdays = new Utils.Weekdays ();
            message ("Set default alarm data: %02d:%02d, placeholder: '%s'", now.get_hour (), now.get_minute (), name_entry.placeholder_text);
        }

        // Ensure button is enabled initially
        validate_input ();
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
        var current_text = name_entry.text;
        message ("Entry changed - text: '%s'", current_text ?? "(null)");
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

        message ("Button sensitivity set to: %s", primary_button.sensitive ? "true" : "false");

        // Could add duplicate checking here if needed
    }

    [GtkCallback]
    private void on_save () {
        message ("Save button clicked");

        int hour = (int) h_spinbutton.value;
        int minute = (int) m_spinbutton.value;

        // Handle null text from He.TextField - try both methods
        string? raw_name = name_entry.text;
        string? raw_name2 = name_entry.get_internal_entry ().text;

        message ("name_entry.text: '%s'", raw_name ?? "(null)");
        message ("name_entry.get_internal_entry().text: '%s'", raw_name2 ?? "(null)");

        string name = "";

        // Use the internal entry text if the direct text is null
        if (raw_name2 != null) {
            name = raw_name2.strip ();
        } else if (raw_name != null) {
            name = raw_name.strip ();
        }

        message ("Hour: %d, Minute: %d, Name: '%s'", hour, minute, name);

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
            if (item != null) {
                item.weekdays = repeats.weekdays;
            }
        }

        if (item != null) {
            message ("About to emit alarm_saved signal");
            alarm_saved (item);
        } else {
            warning ("Failed to create alarm item");
        }
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