const int ADD_ALARM = 1;
const int DELETE_ALARM = 2;

private class Nixie.Duration : Object {
    public int minutes { get; set ; default = 0; }
    public string label { get; set; }

    public Duration (int minutes, string label) {
        this.minutes = minutes;
        this.label = label;
    }
}

private class Nixie.DurationModel : ListModel, Object {
    Duration store[6];

    construct {
        store[0] = new Duration (1, _("1 minute"));
        store[1] = new Duration (5, _("5 minutes"));
        store[2] = new Duration (10, _("10 minutes"));
        store[3] = new Duration (15, _("15 minutes"));
        store[4] = new Duration (20, _("20 minutes"));
        store[5] = new Duration (30, _("30 minutes"));
    }

    public Type get_item_type () {
        return typeof (Duration);
    }

    public uint get_n_items () {
        return 6;
    }

    public Object? get_item (uint n) {
        if (n > 5) {
            return null;
        }
        return store[n];
    }

    public int find_by_duration (int minutes) {
        for (var i = 0; i < get_n_items (); i++) {
            var d = (Duration) get_item (i);
            if (d.minutes == minutes) {
                return i;
            }
        }
        return -1;
    }
}

[GtkTemplate (ui = "/co/tauos/Nixie/alarmface.ui")]
public class Nixie.AlarmFace : He.Bin, Nixie.Utils.Clock {
    private Utils.ContentStore alarms;
    private GLib.Settings settings;
    [GtkChild]
    private unowned Gtk.ListBox listbox;
    [GtkChild]
    private unowned Gtk.ScrolledWindow list_view;

    construct {
        alarms = new Utils.ContentStore ();
        settings = new GLib.Settings ("co.tauos.Nixie");

        var app = (!) GLib.Application.get_default ();
        var action = (GLib.SimpleAction) app.lookup_action ("stop-alarm");
        action.activate.connect ((action, param) => {
            var a = alarms.find ((a) => {
                return ((AlarmItem) a).id == (string) param;
            });

            if (a != null) {
                ((AlarmItem) a).stop ();
            }
        });

        action = (GLib.SimpleAction) app.lookup_action ("snooze-alarm");
        action.activate.connect ((action, param) => {
            var a = alarms.find ((a) => {
                return ((AlarmItem) a).id == (string) param;
            });

            if (a != null) {
                ((AlarmItem) a).snooze ();
            }
        });

        listbox.bind_model (alarms, (item) => {
            item.notify["active"].connect (save);
            return new AlarmRow ((AlarmItem) item, this);
        });

        listbox.row_activated.connect ((row) => {
           var alarm = ((AlarmRow) row).alarm;
           this.edit (alarm);
        });

        load ();

        alarms.items_changed.connect ((position, removed, added) => {
            save ();
        });

        // Start ticking...
        Utils.WallClock.get_default ().tick.connect (() => {
            alarms.foreach ((i) => {
                var a = (AlarmItem)i;
                if (a.tick ()) {
                    if (a.state == AlarmItem.State.RINGING) {
                        ring (a);
                    }
                }
            });
        });
    }

    internal signal void ring (AlarmItem item);

    [GtkCallback]
    private void on_new () {
        activate_new ();
    }

    private void load () {
        alarms.deserialize (settings.get_value ("alarms"), AlarmItem.deserialize);
    }

    private void save () {
        settings.set_value ("alarms", alarms.serialize ());
    }

    internal void edit (AlarmItem alarm) {
        var dialog = new AlarmSetupDialog ((Gtk.Window) get_root (), alarm, alarms);

        // Disable alarm while editing it and remember the original active state.
        alarm.editing = true;

        dialog.primary_button.clicked.connect (() => {
            alarm.editing = false;
            if (dialog.response == ADD_ALARM) {
                ((AlarmSetupDialog) dialog).apply_to_alarm (alarm);
                save ();
            } else if (dialog.response == DELETE_ALARM) {
                alarms.delete_item (alarm);
                save ();
            }
            dialog.destroy ();
        });
        dialog.show ();
    }

    internal void delete (AlarmItem alarm) {
        alarms.delete_item (alarm);
        save ();
    }

    public void activate_new () {
        var dialog = new AlarmSetupDialog ((Gtk.Window) get_root (), null, alarms);
        dialog.primary_button.clicked.connect (() => {
            var alarm = new AlarmItem ();
            ((AlarmSetupDialog) dialog).apply_to_alarm (alarm);
            alarms.add (alarm);
            save ();
            dialog.response = ADD_ALARM;
            dialog.destroy ();
        });
        dialog.show ();
    }
}

[GtkTemplate (ui = "/co/tauos/Nixie/alarmrow.ui")]
private class Nixie.AlarmRow : Gtk.ListBoxRow {
    public AlarmItem alarm { get; construct set; }
    public AlarmFace face { get; construct set; }

    [GtkChild]
    private unowned Gtk.Switch toggle;
    [GtkChild]
    private unowned Gtk.Label title;
    [GtkChild]
    private unowned Gtk.Revealer title_reveal;
    [GtkChild]
    private unowned Gtk.Label time;
    [GtkChild]
    private unowned Gtk.Label repeats;
    [GtkChild]
    private unowned Gtk.Revealer repeats_reveal;

    public AlarmRow (AlarmItem alarm, AlarmFace face) {
        Object (alarm: alarm, face: face);

        alarm.notify["days"].connect (update_repeats);

        alarm.bind_property ("active", toggle, "active", SYNC_CREATE | BIDIRECTIONAL);

        alarm.notify["name"].connect (update);
        alarm.notify["active"].connect (update);
        alarm.notify["state"].connect (update);
        alarm.notify["time"].connect (update);

        update_repeats ();
        update ();
    }

    private void update_repeats () {
        repeats_reveal.reveal_child = !((Utils.Weekdays) alarm.days).empty;
        repeats.label = (string) alarm.days_label;
    }

    private void update () {
        if (alarm.active) {
            add_css_class ("active");
        } else {
            remove_css_class ("active");
        }

        if (alarm.state == AlarmItem.State.SNOOZING) {
            time.label = alarm.snooze_time_label;
        } else {
            time.label = alarm.time_label;
        }

        var label = alarm.name;

        // Prior to 3.36 unamed alarms would just be called "Alarm",
        // pretend alarms called "Alarm" don't have a name (of course
        // this fails if the language/translation has since changed)
        if (alarm.name == _("Alarm")) {
            label = null;
        }

        if (alarm.state == AlarmItem.State.SNOOZING) {
            if (label != null && ((string) label).length > 0) {
                // Translators: The alarm for the time %s titled %s has been "snoozed"
                label = _("Snoozed from %s: %s").printf (alarm.time_label, (string) label);
            } else {
                // Translators: %s is a time
                label = _("Snoozed from %s").printf (alarm.time_label);
            }
        }

        title_reveal.reveal_child = label != null && ((string) label).length > 0;
        title.label = (string) label;
    }

    [GtkCallback]
    private void delete () {
        face.delete (alarm);
    }
}

private struct Nixie.AlarmTime {
    public int hour;
    public int minute;
}

private class Nixie.AlarmItem : Object, Utils.ContentItem {
    // FIXME: should we add a "MISSED" state where the alarm stopped
    // ringing but we keep showing the ringing panel?
    public enum State {
        READY,
        RINGING,
        SNOOZING
    }

    public bool editing { get; set; default = false; }

    public string id { get; construct set; }

    public int snooze_minutes { get; set; default = 10; }

    public int ring_minutes { get; set; default = 5; }

    public string? name {
        get {
            return _name;
        }

        set {
            _name = (string) value;
            setup_bell ();
        }
    }

    public AlarmTime time { get; set; }

    public Utils.Weekdays? days { get; set; }

    public State state { get; private set; }

    public string time_label {
         owned get {
            return Utils.WallClock.get_default ().format_time (alarm_time);
         }
    }

    public string snooze_time_label {
         owned get {
            return Utils.WallClock.get_default ().format_time (snooze_time);
         }
    }

    public string? days_label {
         owned get {
            return days != null ? (string?) ((Utils.Weekdays) days).get_label () : null;
         }
    }

    [CCode (notify = false)]
    public bool active {
        get {
            return _active && !this.editing;
        }

        set {
            if (value != _active) {
                _active = value;
                if (_active) {
                    reset ();
                } else if (state == State.RINGING) {
                    stop ();
                }
                notify_property ("active");
            }
        }
    }

    private string _name;
    private bool _active = true;
    private GLib.DateTime alarm_time;
    private GLib.DateTime snooze_time;
    private GLib.DateTime ring_end_time;
    private Utils.Bell bell;
    private GLib.Notification notification;

    public AlarmItem (string? id = null) {
        var guid = id != null ? (string) id : GLib.DBus.generate_guid ();
        Object (id: guid);
    }

    private void setup_bell () {
        bell = new Utils.Bell ("alarm-clock-elapsed");
        notification = new GLib.Notification (_("Alarm"));
        notification.set_body (name);
        notification.set_priority (HIGH);
        notification.add_button (_("Stop"), "app.stop-alarm::".concat (id));
        notification.add_button (_("Snooze"), "app.snooze-alarm::".concat (id));
    }

    public void reset () {
        update_alarm_time ();
        update_snooze_time (alarm_time);
        state = State.READY;
    }

    private void update_alarm_time () {
        var wallclock = Utils.WallClock.get_default ();
        var now = wallclock.date_time;
        var dt = new GLib.DateTime (wallclock.timezone,
                                    now.get_year (),
                                    now.get_month (),
                                    now.get_day_of_month (),
                                    time.hour,
                                    time.minute,
                                    0);

        if (days == null || ((Utils.Weekdays) days).empty) {
            // Alarm without days.
            if (dt.compare (now) <= 0) {
                // Time already passed, ring tomorrow.
                dt = dt.add_days (1);
            }
        } else {
            // Alarm with at least one day set.
            // Find the next possible day for ringing
            while (dt.compare (now) <= 0 || ! ((Utils.Weekdays) days).get ((Utils.Weekdays.Day) (dt.get_day_of_week () - 1))) {
                dt = dt.add_days (1);
            }
        }

        alarm_time = dt;
    }

    private void update_snooze_time (GLib.DateTime start_time) {
        snooze_time = start_time.add_minutes (snooze_minutes);
    }

    public virtual signal void ring () {
        var app = (Nixie.Application) GLib.Application.get_default ();
        app.send_notification ("alarm-clock-elapsed", notification);
        bell.ring ();
    }

    private void start_ringing (GLib.DateTime now) {
        update_snooze_time (now);
        ring_end_time = now.add_minutes (ring_minutes);
        state = State.RINGING;
        ring ();
    }

    public void snooze () {
        bell.stop ();
        state = State.SNOOZING;
    }

    public void stop () {
        bell.stop ();
        update_snooze_time (alarm_time);
        state = State.READY;
    }

    private bool compare_with_item (AlarmItem i) {
        return (this.alarm_time.compare (i.alarm_time) == 0 && (this.active || this.editing) && i.active);
    }

    public bool check_duplicate_alarm (List<AlarmItem> alarms) {
        update_alarm_time ();

        foreach (var item in alarms) {
            if (this.compare_with_item (item)) {
                return true;
            }
        }
        return false;
    }

    // Update the state and ringing time. Ring or stop
    // depending on the current time.
    // Returns true if the state changed, false otherwise.
    public bool tick () {
        if (!active) {
            return false;
        }

        State last_state = state;

        var wallclock = Utils.WallClock.get_default ();
        var now = wallclock.date_time;

        if (state == State.RINGING && now.compare (ring_end_time) > 0) {
            stop ();
        }

        if (state == State.SNOOZING && now.compare (snooze_time) > 0) {
            start_ringing (now);
        }

        if (state == State.READY && now.compare (alarm_time) > 0) {
            start_ringing (now);
            update_alarm_time (); // reschedule for the next repeat
        }

        return state != last_state;
    }

    public void serialize (GLib.VariantBuilder builder) {
        builder.open (new GLib.VariantType ("a{sv}"));
        builder.add ("{sv}", "name", new GLib.Variant.string ((string) name));
        builder.add ("{sv}", "id", new GLib.Variant.string (id));
        builder.add ("{sv}", "active", new GLib.Variant.boolean (active));
        builder.add ("{sv}", "hour", new GLib.Variant.int32 (time.hour));
        builder.add ("{sv}", "minute", new GLib.Variant.int32 (time.minute));
        builder.add ("{sv}", "days", ((Utils.Weekdays) days).serialize ());
        builder.add ("{sv}", "snooze_minutes", new GLib.Variant.int32 (snooze_minutes));
        builder.add ("{sv}", "ring_minutes", new GLib.Variant.int32 (ring_minutes));
        builder.close ();
    }

    public static Utils.ContentItem? deserialize (Variant alarm_variant) {
        string key;
        Variant val;
        string? name = null;
        string? id = null;
        bool active = true;
        int hour = -1;
        int minute = -1;
        int snooze_minutes = 10;
        int ring_minutes = 5;
        Utils.Weekdays? days = null;

        var iter = alarm_variant.iterator ();
        while (iter.next ("{sv}", out key, out val)) {
            if (key == "name") {
                name = (string) val;
            } else if (key == "id") {
                id = (string) val;
            } else if (key == "active") {
                active = (bool) val;
            } else if (key == "hour") {
                hour = (int32) val;
            } else if (key == "minute") {
                minute = (int32) val;
            } else if (key == "days") {
                days = Utils.Weekdays.deserialize (val);
            } else if (key == "snooze_minutes") {
                snooze_minutes = (int32) val;
            } else if (key == "ring_minutes") {
                ring_minutes = (int32) val;
            }
        }

        if (hour >= 0 && minute >= 0) {
            AlarmItem alarm = new AlarmItem (id);
            alarm.name = name;
            alarm.active = active;
            alarm.time = { hour, minute };
            alarm.days = days;
            alarm.ring_minutes = ring_minutes;
            alarm.snooze_minutes = snooze_minutes;
            alarm.reset ();
            return alarm;
        } else {
            warning ("Invalid alarm %s", name != null ? (string) name : "[unnamed]");
        }

        return null;
    }
}

[GtkTemplate (ui = "/co/tauos/Nixie/alarmdaypickerrow.ui")]
public class Nixie.AlarmDayPickerRow : He.Bin {
    public bool monday {
        get {
            return days[Utils.Weekdays.Day.MON];
        }

        set {
            days[Utils.Weekdays.Day.MON] = value;
            update ();
        }
    }

    public bool tuesday {
        get {
            return days[Utils.Weekdays.Day.TUE];
        }

        set {
            days[Utils.Weekdays.Day.TUE] = value;
            update ();
        }
    }

    public bool wednesday {
        get {
            return days[Utils.Weekdays.Day.WED];
        }

        set {
            days[Utils.Weekdays.Day.WED] = value;
            update ();
        }
    }

    public bool thursday {
        get {
            return days[Utils.Weekdays.Day.THU];
        }

        set {
            days[Utils.Weekdays.Day.THU] = value;
            update ();
        }
    }

    public bool friday {
        get {
            return days[Utils.Weekdays.Day.FRI];
        }

        set {
            days[Utils.Weekdays.Day.FRI] = value;
            update ();
        }
    }

    public bool saturday {
        get {
            return days[Utils.Weekdays.Day.SAT];
        }

        set {
            days[Utils.Weekdays.Day.SAT] = value;
            update ();
        }
    }

    public bool sunday {
        get {
            return days[Utils.Weekdays.Day.SUN];
        }

        set {
            days[Utils.Weekdays.Day.SUN] = value;
            update ();
        }
    }

    public signal void days_changed ();

    private Utils.Weekdays days = new Utils.Weekdays ();

    [GtkChild]
    private unowned Gtk.Box box;

    construct {
        // Create actions to control propeties from menu items
        var group = new SimpleActionGroup ();
        group.add_action (new PropertyAction ("day-0", this, "monday"));
        group.add_action (new PropertyAction ("day-1", this, "tuesday"));
        group.add_action (new PropertyAction ("day-2", this, "wednesday"));
        group.add_action (new PropertyAction ("day-3", this, "thursday"));
        group.add_action (new PropertyAction ("day-4", this, "friday"));
        group.add_action (new PropertyAction ("day-5", this, "saturday"));
        group.add_action (new PropertyAction ("day-6", this, "sunday"));
        insert_action_group ("repeats", group);

        // Create an array with the weekday items with
        // buttons[0] referencing the button for Monday, and so on.
        var buttons = new Gtk.ToggleButton[7];
        for (int i = 0; i < 7; i++) {
            var day = (Utils.Weekdays.Day) i;
            buttons[i] = new Gtk.ToggleButton.with_label (day.symbol ());
            buttons[i].action_name = "repeats.day-%i".printf (i);
            buttons[i].tooltip_text = day.name ();
            buttons[i].add_css_class ("circular");
            buttons[i].halign = Gtk.Align.START;
        }

        // Add the items, starting with the first day of the week
        // depending on the locale.
        var first_weekday = Utils.Weekdays.Day.get_first_weekday ();
        for (int i = 0; i < 7; i++) {
            var day_number = (first_weekday + i) % 7;

            box.append (buttons[day_number]);
        }

        update ();
    }

    public void load (Utils.Weekdays current_days) {
        // Copy in the days
        for (int i = 0; i < 7; i++) {
            days[(Utils.Weekdays.Day) i] = current_days[(Utils.Weekdays.Day) i];
        }

        // Make sure the buttons update
        notify_property ("monday");
        notify_property ("tuesday");
        notify_property ("wednesday");
        notify_property ("thursday");
        notify_property ("friday");
        notify_property ("saturday");
        notify_property ("sunday");

        update ();
    }

    public Utils.Weekdays store () {
        var new_days = new Utils.Weekdays ();

        for (int i = 0; i < 7; i++) {
            new_days[(Utils.Weekdays.Day) i] = days[(Utils.Weekdays.Day) i];
        }

        return new_days;
    }

    private void update () {
        days_changed ();
    }
}

[GtkTemplate (ui = "/co/tauos/Nixie/alarmringingpanel.ui")]
private class Nixie.AlarmRingingPanel : He.Bin {
    public AlarmItem? alarm {
        get {
            return _alarm;
        }
        set {
            if (_alarm != null) {
                ((AlarmItem) _alarm).disconnect (alarm_state_handler);
            }

            _alarm = value;

            if (_alarm != null) {
                alarm_state_handler = ((AlarmItem) _alarm).notify["state"].connect (() => {
                    if (((AlarmItem) _alarm).state != AlarmItem.State.RINGING) {
                        dismiss ();
                    }
                });

                stop_button.action_target = ((AlarmItem) _alarm).id;
                stop_button.action_name = "app.stop-alarm";

                snooze_button.action_target = ((AlarmItem) _alarm).id;
                snooze_button.action_name = "app.snooze-alarm";
            }

            update ();
        }
    }

    private AlarmItem? _alarm;
    private ulong alarm_state_handler;
    [GtkChild]
    private unowned Gtk.Label title_label;
    [GtkChild]
    private unowned Gtk.Label time_label;
    [GtkChild]
    private unowned Gtk.Button stop_button;
    [GtkChild]
    private unowned Gtk.Button snooze_button;

    construct {
        // Start ticking...
        Utils.WallClock.get_default ().tick.connect (update);
    }

    public virtual signal void dismiss () {
        alarm = null;
    }

    private void update () {
        if (alarm != null) {
            title_label.label = (string) ((AlarmItem) alarm).name;
            if (((AlarmItem) alarm).state == SNOOZING) {
                time_label.label = ((AlarmItem) alarm).snooze_time_label;
            } else {
                time_label.label = ((AlarmItem) alarm).time_label;
            }
        } else {
            title_label.label = "";
            time_label.label = "";
        }
    }
}

[GtkTemplate (ui = "/co/tauos/Nixie/alarmsetupdialog.ui")]
private class Nixie.AlarmSetupDialog : He.Window {
    private Utils.WallClock.Format format;
    [GtkChild]
    private unowned Gtk.Box time_box;
    [GtkChild]
    private unowned Gtk.SpinButton h_spinbutton;
    [GtkChild]
    private unowned Gtk.SpinButton m_spinbutton;
    [GtkChild]
    private unowned Gtk.Entry name_entry;
    [GtkChild]
    private unowned AlarmDayPickerRow repeats;
    [GtkChild]
    private unowned Gtk.Revealer label_revealer;
    [GtkChild]
    public unowned He.PillButton primary_button;
    [GtkChild]
    private unowned Gtk.Button delete_button;
    private List<AlarmItem> other_alarms;
    private DurationModel duration_model;
    public int response;

    static construct {
        typeof (AlarmDayPickerRow).ensure ();
        typeof (Duration).ensure ();
    }

    public AlarmSetupDialog (Gtk.Window parent, AlarmItem? alarm, ListModel all_alarms) {
        Object (
            transient_for: parent,
            title: alarm != null ? _("Edit Alarm") : _("New Alarm")
        );

        if (alarm != null) {
            this.primary_button.label = (_("Done"));
        } else {
            this.primary_button.label = (_("Add"));
        }

        delete_button.visible = alarm != null;

        other_alarms = new List<AlarmItem> ();
        var n = all_alarms.get_n_items ();
        for (int i = 0; i < n; i++) {
            var item = (AlarmItem) all_alarms.get_object (i);
            if (alarm != item) {
                other_alarms.prepend ((AlarmItem) all_alarms.get_object (i));
            }
        }

        // Force LTR since we do not want to reverse [hh] : [mm]
        time_box.set_direction (Gtk.TextDirection.LTR);

        format = Utils.WallClock.get_default ().format;

        set_from_alarm (alarm);
    }

    private static string duration_label (Duration item) {
        return item.label;
    }

    public void set_from_alarm (AlarmItem? alarm) {
        string? name;
        bool active;
        int hour;
        int minute;
        unowned Utils.Weekdays? days;

        if (alarm == null) {
            var wc = Utils.WallClock.get_default ();
            name = "";
            hour = wc.date_time.get_hour ();
            minute = wc.date_time.get_minute ();
            days = null;
            active = true;
        } else {
            name = ((AlarmItem) alarm).name;
            hour = ((AlarmItem) alarm).time.hour;
            minute = ((AlarmItem) alarm).time.minute;
            days = ((AlarmItem) alarm).days;
            active = ((AlarmItem) alarm).active;
        }

        h_spinbutton.set_value (hour);
        m_spinbutton.set_value (minute);

        // Set the name.
        name_entry.set_text ((string) name);

        if (days != null) {
            repeats.load ((Utils.Weekdays) days);
        }
    }

    // Sets alarm according to the current dialog settings.
    public void apply_to_alarm (AlarmItem alarm) {
        var name = name_entry.get_text ();
        var hour = h_spinbutton.get_value_as_int ();
        var minute = m_spinbutton.get_value_as_int ();

        AlarmTime time = { hour, minute };

        var days = repeats.store ();

        alarm.freeze_notify ();

        alarm.name = name;
        alarm.time = time;
        alarm.days = days;

        // Force update of alarm_time before notifying the changes
        alarm.reset ();
        alarm.thaw_notify ();
    }

    private void avoid_duplicate_alarm () {
        var alarm = new AlarmItem ();
        apply_to_alarm (alarm);

        var duplicate = alarm.check_duplicate_alarm (other_alarms);
        this.set_sensitive (!duplicate);
        label_revealer.set_reveal_child (duplicate);
    }

    [GtkCallback]
    private void days_changed () {
        avoid_duplicate_alarm ();
    }

    [GtkCallback]
    private void entry_changed (Gtk.Editable editable) {
        avoid_duplicate_alarm ();
    }

    [GtkCallback]
    private void spinbuttons_changed (Gtk.Editable editable) {
        avoid_duplicate_alarm ();
    }

    [GtkCallback]
    private bool show_leading_zeros (Gtk.SpinButton spin_button) {
        spin_button.set_text ("%02i".printf (spin_button.get_value_as_int ()));
        return true;
    }

    [GtkCallback]
    private void delete () {
        response = DELETE_ALARM;
    }
}