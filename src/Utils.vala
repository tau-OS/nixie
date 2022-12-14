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

// C stuff
extern int cutils_get_week_start ();
extern bool calculate_sunrise_sunset (double lat,
                                      double lon,
                                      int year,
                                      int month,
                                      int day,
                                      double correction,
                                      out int rise_hour,
                                      out int rise_min,
                                      out int set_hour,
                                      out int set_min);
const double RISESET_CORRECTION_NONE = 0.0;
const double RISESET_CORRECTION_CIVIL = 6.0;
//

public class Geo.Info : Object {
    public GClue.Location? geo_location { get; private set; default = null; }

    private GWeather.Location? found_location;
    private string? country_code;
    private GClue.Simple simple;
    private double minimal_distance;

    public signal void location_changed (GWeather.Location location);

    public Info () {
        country_code = null;
        found_location = null;
        minimal_distance = 1000.0d;
    }

    public async void seek () {
        try {
            simple = yield new GClue.Simple (Config.APP_ID, GClue.AccuracyLevel.CITY, null);
        } catch (Error e) {
            warning ("Failed to connect to GeoClue2 service: %s", e.message);
            return;
        }

        simple.notify["location"].connect (() => {
            on_location_updated.begin ();
        });

        on_location_updated.begin ();
    }

    public async void on_location_updated () {
        geo_location = simple.get_location ();

        yield seek_country_code ();

        yield search_locations ((GWeather.Location) GWeather.Location.get_world ());

        if (found_location != null) {
            location_changed ((GWeather.Location) found_location);
        }
    }

    private async void seek_country_code () requires (geo_location != null) {
        var location = new Geocode.Location (((GClue.Location) geo_location).latitude,
                                                ((GClue.Location) geo_location).longitude);
        var reverse = new Geocode.Reverse.for_location (location);

        try {
            var place = yield reverse.resolve_async ();

            country_code = place.get_country_code ();
        } catch (Error e) {
            warning ("Failed to obtain country code: %s", e.message);
        }
    }

    private double deg_to_rad (double deg) {
        return Math.PI / 180.0d * deg;
    }

    private double get_distance (double latitude1, double longitude1, double latitude2, double longitude2) {
        const double EARTH_RADIUS = 6372.795d;

        double lat1 = deg_to_rad (latitude1);
        double lat2 = deg_to_rad (latitude2);
        double lon1 = deg_to_rad (longitude1);
        double lon2 = deg_to_rad (longitude2);

        return Math.acos (Math.cos (lat1) * Math.cos (lat2) * Math.cos (lon1 - lon2) +
                            Math.sin (lat1) * Math.sin (lat2)) * EARTH_RADIUS;
    }

    private async void search_locations (GWeather.Location location) requires (geo_location != null) {
        if (this.country_code != null) {
            string? loc_country_code = location.get_country ();
            if (loc_country_code != null) {
                if (loc_country_code != this.country_code) {
                    return;
                }
            }
        }

        var loc = location.next_child (null);
        while (loc != null) {
            if (loc.get_level () == GWeather.LocationLevel.CITY) {
                if (loc.has_coords ()) {
                    double latitude, longitude, distance;

                    loc.get_coords (out latitude, out longitude);
                    distance = get_distance (((GClue.Location) geo_location).latitude,
                                                ((GClue.Location) geo_location).longitude,
                                                latitude,
                                                longitude);

                    if (distance < minimal_distance) {
                        found_location = loc;
                        minimal_distance = distance;
                    }
                }
            }

            yield search_locations (loc);
            loc = location.next_child (loc);
        }
    }

    public bool is_location_similar (GWeather.Location location) {
        if (this.found_location != null) {
            var country_code = location.get_country ();
            var found_country_code = ((GWeather.Location) found_location).get_country ();
            if (country_code != null && country_code == found_country_code) {
                var timezone = location.get_timezone ();
                var found_timezone = ((GWeather.Location) found_location).get_timezone ();

                if (timezone != null && found_timezone != null) {
                    var tzid = timezone.get_identifier ();
                    var found_tzid = found_timezone.get_identifier ();
                    if (tzid == found_tzid) {
                        return true;
                    }
                }
            }
        }

        return false;
    }
}

public interface Nixie.Utils.Clock : GLib.Object {
    public virtual void activate_new () {
    }

    public virtual bool escape_pressed () {
        return false;
    }
}

public interface Nixie.Utils.ContentItem : GLib.Object {
    public abstract string? name { get; set; }
    public abstract void serialize (GLib.VariantBuilder builder);
}

public class Nixie.Utils.ContentStore : GLib.Object, GLib.ListModel {
    private ListStore store;
    private CompareDataFunc<ContentItem>? sort_func;


    public ContentStore () {
        store = new ListStore (typeof (ContentItem));
        store.items_changed.connect ((position, removed, added) => {
            items_changed (position, removed, added);
        });
    }

    public Type get_item_type () {
        return store.get_item_type ();
    }

    public uint get_n_items () {
        return store.get_n_items ();
    }

    public Object? get_item (uint position) {
        return store.get_item (position);
    }

    public void set_sorting (owned CompareDataFunc<ContentItem> sort) {
        sort_func = (owned) sort;

        // TODO: we should re-sort, but for now we only
        // set this before adding any item
        assert (store.get_n_items () == 0);
    }

    public void add (ContentItem item) {
        if (sort_func == null) {
            store.append (item);
        } else {
            store.insert_sorted (item, sort_func);
        }
    }

    public void prepend (ContentItem item) {
        store.insert (0, item);
    }

    public int get_index (ContentItem item) {
        int position = -1;
        var n = store.get_n_items ();
        for (int i = 0; i < n; i++) {
            var compared_item = (ContentItem) store.get_object (i);
            if (compared_item == item) {
                position = i;
                break;
            }
        }
        return position;
    }

    public void remove (ContentItem item) {
        var index = get_index (item);
        if (index != -1) {
            store.remove (index);
        }
    }

    public delegate void ForeachFunc (ContentItem item);

    public void foreach (ForeachFunc func) {
        var n = store.get_n_items ();
        for (int i = 0; i < n; i++) {
            func ((ContentItem) store.get_object (i));
        }
    }

    public delegate bool FindFunc (ContentItem item);

    public ContentItem? find (FindFunc func) {
        var n = store.get_n_items ();
        for (int i = 0; i < n; i++) {
            var item = (ContentItem) store.get_object (i);
            if (func (item)) {
                return item;
            }
        }
        return null;
    }

    public void delete_item (ContentItem item) {
        var n = store.get_n_items ();
        for (int i = 0; i < n; i++) {
            var o = store.get_object (i);
            if (o == item) {
                store.remove (i);

                if (sort_func != null) {
                    store.sort (sort_func);
                }

                return;
            }
        }
    }

    public Variant serialize () {
        var builder = new GLib.VariantBuilder (new VariantType ("aa{sv}"));
        var n = store.get_n_items ();
        for (int i = 0; i < n; i++) {
            ((ContentItem) store.get_object (i)).serialize (builder);
        }
        return builder.end ();
    }

    public delegate ContentItem? DeserializeItemFunc (Variant v);

    public void deserialize (Variant variant, DeserializeItemFunc deserialize_item) {
        Variant item;
        var iter = variant.iterator ();
        while (iter.next ("@a{sv}", out item)) {
            ContentItem? i = deserialize_item (item);
            if (i != null) {
                add ((ContentItem) i);
            }
        }
    }
}

namespace Nixie.Utils.Misc {
    public void time_to_hms (double t, out int h, out int m, out int s, out double remainder) {
        h = (int) t / 3600;
        t = t % 3600;
        m = (int) t / 60;
        t = t % 60;
        s = (int) t;
        remainder = t - s;
    }

    private string render_duration (double duration) {
        int h;
        int m;
        int s;
        double r;
        time_to_hms (Math.floor (duration * 100) / 100, out h, out m, out s, out r);
        int cs = (int) (r * 10);
        return "%02i\u200E ∶ %02i\u200E ∶ %02i. %i".printf (h.abs (), m.abs (), s.abs (), cs.abs ());
    }
}

public class Nixie.Utils.Weekdays {
    private static string[]? abbreviations = null;
    private static string[]? names = null;

    public enum Day {
        MON = 0,
        TUE,
        WED,
        THU,
        FRI,
        SAT,
        SUN;

        private const string[] SYMBOLS = {
            // Translators: This is used in the repeat toggle for Monday
            NC_("Alarm|Repeat-On|Monday", "M"),
            // Translators: This is used in the repeat toggle for Tuesday
            NC_("Alarm|Repeat-On|Tuesday", "T"),
            // Translators: This is used in the repeat toggle for Wednesday
            NC_("Alarm|Repeat-On|Wednesday", "W"),
            // Translators: This is used in the repeat toggle for Thursday
            NC_("Alarm|Repeat-On|Thursday", "T"),
            // Translators: This is used in the repeat toggle for Friday
            NC_("Alarm|Repeat-On|Friday", "F"),
            // Translators: This is used in the repeat toggle for Saturday
            NC_("Alarm|Repeat-On|Saturday", "S"),
            // Translators: This is used in the repeat toggle for Sunday
            NC_("Alarm|Repeat-On|Sunday", "S")
        };

        private const string[] EN_DAYS = {
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday"
        };

        private const string[] PLURALS = {
            N_("Mondays"),
            N_("Tuesdays"),
            N_("Wednesdays"),
            N_("Thursdays"),
            N_("Fridays"),
            N_("Saturdays"),
            N_("Sundays")
        };

        public string symbol () {
            return dpgettext2 (null, "Alarm|Repeat-On|" + EN_DAYS[this], SYMBOLS[this]);
        }

        public string plural () {
            return _(PLURALS[this]);
        }

        public string abbreviation () {
            // lazy init because we cannot rely on class init being
            // called for us (at least in the current version of vala)
            if (abbreviations == null) {
                abbreviations = {
                     (new GLib.DateTime.utc (1, 1, 1, 0, 0, 0)).format ("%a"),
                     (new GLib.DateTime.utc (1, 1, 2, 0, 0, 0)).format ("%a"),
                     (new GLib.DateTime.utc (1, 1, 3, 0, 0, 0)).format ("%a"),
                     (new GLib.DateTime.utc (1, 1, 4, 0, 0, 0)).format ("%a"),
                     (new GLib.DateTime.utc (1, 1, 5, 0, 0, 0)).format ("%a"),
                     (new GLib.DateTime.utc (1, 1, 6, 0, 0, 0)).format ("%a"),
                     (new GLib.DateTime.utc (1, 1, 7, 0, 0, 0)).format ("%a"),
                };
            }
            return abbreviations[this];
        }

        public string name () {
            // lazy init because we cannot rely on class init being
            // called for us (at least in the current version of vala)
            if (names == null) {
                names = {
                     (new GLib.DateTime.utc (1, 1, 1, 0, 0, 0)).format ("%A"),
                     (new GLib.DateTime.utc (1, 1, 2, 0, 0, 0)).format ("%A"),
                     (new GLib.DateTime.utc (1, 1, 3, 0, 0, 0)).format ("%A"),
                     (new GLib.DateTime.utc (1, 1, 4, 0, 0, 0)).format ("%A"),
                     (new GLib.DateTime.utc (1, 1, 5, 0, 0, 0)).format ("%A"),
                     (new GLib.DateTime.utc (1, 1, 6, 0, 0, 0)).format ("%A"),
                     (new GLib.DateTime.utc (1, 1, 7, 0, 0, 0)).format ("%A"),
                };
            }
            return names[this];
        }

        public static Day get_first_weekday () {
            var d = cutils_get_week_start ();
            return (Day) ((d + 6) % 7);
        }
    }

    private const bool[] WEEKDAYS = {
        true, true, true, true, true, false, false
    };

    private const bool[] WEEKENDS = {
        false, false, false, false, false, true, true
    };

    const bool[] NONE = {
        false, false, false, false, false, false, false
    };

    const bool[] ALL = {
        true, true, true, true, true, true, true
    };

    private bool[] days = NONE;

    public bool empty {
        get {
            return (days_equal (NONE));
        }
    }

    public bool is_weekdays {
        get {
            return (days_equal (WEEKDAYS));
        }
    }

    public bool is_weekends {
        get {
            return (days_equal (WEEKENDS));
        }
    }

    public bool is_all {
        get {
            return (days_equal (ALL));
        }
    }

    private bool days_equal (bool[] d) {
        assert (d.length == 7);
        return (Memory.cmp (d, days, days.length * sizeof (bool)) == 0);
    }

    public bool get (Day d) {
        assert (d >= 0 && d < 7);
        return days[d];
    }

    public void set (Day d, bool on) {
        assert (d >= 0 && d < 7);
        days[d] = on;
    }

    public string get_label () {
        string? r = null;
        int n = 0;
        int first = -1;
        for (int i = 0; i < 7; i++) {
            if (get ((Day) i)) {
                if (first < 0) {
                    first = i;
                }
                n++;
            }
        }

        if (n == 0) {
            r = "";
        } else if (n == 1) {
            r = ((Day) first).plural ();
        } else if (n == 7) {
            r = _("Every Day");
        } else if (days_equal (WEEKDAYS)) {
            r = _("Weekdays");
        } else if (days_equal (WEEKENDS)) {
            r = _("Weekends");
        } else {
            string?[]? abbrs = {};
            for (int i = 0; i < 7; i++) {
                Day d = (Day.get_first_weekday () + i) % 7;
                if (get (d)) {
                    abbrs += d.abbreviation ();
                }
            }
            r = string.joinv (", ", abbrs);
        }
        return (string) r;
    }

    // Note that we serialze days according to ISO 8601
    // (1 is Monday, 2 is Tuesday... 7 is Sunday)

    public GLib.Variant serialize () {
        var builder = new GLib.VariantBuilder (new VariantType ("ai"));
        int32 i = 1;
        foreach (var d in days) {
            if (d) {
                builder.add ("i", i);
            }
            i++;
        }
        return builder.end ();
    }

    public static Weekdays deserialize (GLib.Variant days_variant) {
        Weekdays d = new Weekdays ();
        foreach (var v in days_variant) {
            var i = (int32) v;
            if (i > 0 && i <= 7) {
                d.set ((Day) (i - 1), true);
            } else {
                warning ("Invalid days %d", i);
            }
        }
        return d;
    }
}

public class Nixie.Utils.Bell : Object {
    private GSound.Context? gsound;
    private GLib.Cancellable cancellable;
    private string soundtheme;
    private string sound;

    public Bell (string soundid) {
        try {
            gsound = new GSound.Context ();
        } catch (GLib.Error e) {
            warning ("Sound could not be initialized, error: %s", e.message);
        }

        var settings = new GLib.Settings ("org.gnome.desktop.sound");
        soundtheme = settings.get_string ("theme-name");
        sound = soundid;
        cancellable = new GLib.Cancellable ();
    }

    private async void ring_real (bool repeat) {
        if (gsound == null) {
            return;
        }

        if (cancellable.is_cancelled ()) {
            cancellable.reset ();
        }

        try {
            do {
                yield ((GSound.Context) gsound).play_full (cancellable,
                                                           GSound.Attribute.EVENT_ID, sound,
                                                           GSound.Attribute.CANBERRA_XDG_THEME_NAME, soundtheme,
                                                           GSound.Attribute.MEDIA_ROLE, "alarm");
            } while (repeat);
        } catch (GLib.IOError.CANCELLED e) {
            // ignore
        } catch (GLib.Error e) {
            warning ("Error playing sound: %s", e.message);
        }
    }

    public void ring_once () {
        ring_real.begin (false);
    }

    public void ring () {
        ring_real.begin (true);
    }

    public void stop () {
        cancellable.cancel ();
    }
}

public class Nixie.Utils.WallClock : Object {
    public enum Format {
        TWELVE,
        TWENTYFOUR
    }

    private static WallClock? instance;

    public static WallClock get_default () {
        if (instance == null) {
            instance = new WallClock ();
        }
        // If it's still null something has gone horribly wrong
        return (WallClock) instance;
    }

    public GLib.DateTime date_time { get; private set; }
    public GLib.TimeZone timezone { get; private set; }
    public Format format { get; private set; }

    private Gnome.WallClock wc;
    private Portal.Settings portal;

    private WallClock () {
        wc = new Gnome.WallClock ();
        wc.notify["clock"].connect (() => {
            update ();
            tick ();
        });

        // mirror the wallclock's timezone property
        timezone = wc.timezone;
        wc.notify["timezone"].connect (() => {
            timezone = wc.timezone;
        });

        // system-wide settings about clock format
        portal_run.begin ();
        update ();
    }

    construct {
        try {
            portal = Portal.Settings.get ();
            portal.setting_changed.connect ((scheme, key, value) => {
                if (scheme == "org.gnome.desktop.interface" && key == "clock-format") {
                    var format = value.get_string ();
                    update_format (format);
                    update ();
                    format_time (date_time);
                }
            });
        } catch (GLib.Error error) {
            warning ("Failed to request time format: %s", error.message);
        }
    }

    public signal void tick ();

    private void update_format (string sys_format) {
        format = sys_format == "12h" ? Format.TWELVE : Format.TWENTYFOUR;
    }

    private async void portal_run () {
        try {
            portal = Portal.Settings.get ();
            var variant = portal.read ("org.gnome.desktop.interface", "clock-format").get_variant ();
            var format = variant.get_string ();

            update_format (format);
            update ();
            format_time (date_time);
            yield;
        } catch (GLib.Error error) {
            warning ("Failed to request time format: %s", error.message);
        }
    }

    // provide various types/objects of the same time, to be used directly
    // in AlarmItem and ClockItem, so they don't need to call these
    // functions themselves all the time (they only care about minutes).
    private void update () {
        date_time = new GLib.DateTime.now (timezone);
    }

    public string format_time (GLib.DateTime date_time) {
        update ();
        string time = date_time.format (format == Format.TWELVE ? "%I:%M %p" : "%H:%M");

        // Replace ":" with ratio, space with thin-space, and prepend LTR marker
        // to force direction. Replacement is done afterward because date_time.format
        // may fail with utf8 chars in some locales
        time = time.replace (":", "\xE2\x80\x8E\xE2\x88\xB6");

        if (format == Format.TWELVE) {
            time = time.replace (" ", "\xE2\x80\x89");
        }

        return time;
    }
}