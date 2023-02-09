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

[GtkTemplate (ui = "/com/fyralabs/Nixie/worldface.ui")]
public class Nixie.WorldFace : He.Bin, Nixie.Utils.Clock {
    public MainWindow win {get; set;}

    private Utils.ContentStore locations;
    private GLib.Settings settings;

    [GtkChild]
    private unowned Gtk.ListBox listbox;

    construct {
        locations = new Utils.ContentStore ();
        settings = new GLib.Settings ("com.fyralabs.Nixie");

        locations.set_sorting ((item1, item2) => {
            var interval1 = ((WorldItem) item1).location.get_timezone ().find_interval (GLib.TimeType.UNIVERSAL, Gdk.CURRENT_TIME);
            var offset1 = ((WorldItem) item1).location.get_timezone ().get_offset (interval1);
            var interval2 = ((WorldItem) item2).location.get_timezone ().find_interval (GLib.TimeType.UNIVERSAL, Gdk.CURRENT_TIME);
            var offset2 = ((WorldItem) item2).location.get_timezone ().get_offset (interval2);
            if (offset1 < offset2)
                return -1;
            if (offset1 > offset2)
                return 1;
            return 0;
        });

        listbox.bind_model (locations, (item) => {
            var row = new WorldRow ((WorldItem) item);
            row.remove_clock.connect (() => remove_clock ((WorldItem) item));
            return row;
        });

        load ();

        if (settings.get_boolean ("geolocation")) {
            use_geolocation.begin ((obj, res) => {
                use_geolocation.end (res);
            });
        }

        locations.items_changed.connect ((position, removed, added) => {
            save ();
        });

        // Start ticking...
        Utils.WallClock.get_default ().tick.connect (() => {
            locations.foreach ((l) => {
                ((WorldItem)l).tick ();
            });
            // TODO Only need to queue what changed
            listbox.queue_draw ();
        });
    }

    [GtkCallback]
    private void on_new () {
        activate_new ();
    }

    private void load () {
        locations.deserialize (settings.get_value ("clocks"), WorldItem.deserialize);

        use_geolocation.begin ((obj, res) => {
            use_geolocation.end (res);
        });
    }

    private void save () {
        settings.set_value ("clocks", locations.serialize ());
    }

    private void remove_clock (WorldItem item) {
        locations.remove (item);
    }

    private async void use_geolocation () {
        Geo.Info geo_info = new Geo.Info ();

        geo_info.location_changed.connect ((found_location) => {
            var item = (WorldItem?) locations.find ((l) => {
                return geo_info.is_location_similar (((WorldItem) l).location);
            });

            if (item != null) {
                return;
            }

            var auto_item = new WorldItem (found_location);
            auto_item.automatic = true;
            locations.prepend (auto_item);
        });

        yield geo_info.seek ();
    }

    private void add_location_item (WorldItem item) {
        locations.add (item);
        save ();
    }

    public bool location_exists (GWeather.Location location) {
        var exists = false;
        var n = locations.get_n_items ();
        for (int i = 0; i < n; i++) {
            var l = (WorldItem) locations.get_object (i);
            if (l.location.equal (location)) {
                exists = true;
                break;
            }
        }

        return exists;
    }

    public void add_location (GWeather.Location location) {
        if (!location_exists (location)) {
            add_location_item (new WorldItem (location));
        }
    }

    public void activate_new () {
        var dialog = new WorldLocationFinder ((Gtk.Window) get_root (), this);

        dialog.location_added.connect (() => {
                var location = dialog.get_selected_location ();
                if (location != null)
                    add_location ((GWeather.Location) location);

                dialog.destroy ();
            });
        dialog.present ();
    }
}

[GtkTemplate (ui = "/com/fyralabs/Nixie/worldrow.ui")]
private class Nixie.WorldRow : He.Bin {
    public WorldItem location { get; construct set; }

    [GtkChild]
    private unowned Gtk.Label block_title;
    [GtkChild]
    private unowned Gtk.Label block_subtitle;
    [GtkChild]
    private unowned Gtk.Widget delete_button;

    internal signal void remove_clock ();

    public bool automatic {get; set;}

    public WorldRow (WorldItem location) {
        Object (location: location);

        location.bind_property ("city-name", block_subtitle, "label", BindingFlags.DEFAULT | BindingFlags.SYNC_CREATE);
        location.tick.connect (update);

        update ();
    }

    private void update () {
        remove_css_class ("night");
        remove_css_class ("civil");
        remove_css_class ("day");
        add_css_class (location.state_class);

        if (location.automatic) {
            add_css_class ("automatic");
        }

        if (location.day_label != null && location.day_label != "") {
            block_title.label = "%s".printf ((string) location.day_label);
            delete_button.visible = true;
            delete_button.remove_css_class ("hidden");
        } else if (location.automatic) {
            // Translators: This clock represents the local time
            block_title.label = _("Current location");
            delete_button.visible = false;
            delete_button.add_css_class ("hidden");
        } else {
            delete_button.visible = true;
            delete_button.remove_css_class ("hidden");
        }

        block_title.label = location.time_label;
    }

    [GtkCallback]
    private void delete () {
        remove_clock ();
    }
}

public class Nixie.WorldItem : Object, Nixie.Utils.ContentItem {
    public GWeather.Location location { get; set; }

    public bool automatic { get; set; default = false; }

    public string? name {
        get {
            // We store it in a _name member even if we overwrite it every time
            // since the abstract name property does not return an owned string
            if (country_name != null) {
                if (state_name != null) {
                    _name = "%s, %s, %s".printf (city_name, (string) state_name, (string) country_name);
                } else {
                    _name = "%s, %s".printf (city_name, (string) country_name);
                }
            } else {
                _name = city_name;
            }

            return _name;
        }
        set {
            // ignored
        }
    }

    public string city_name {
        owned get {
            var city_name = location.get_city_name ();
            /* Named Timezones don't have city names */
            if (city_name == null) {
                city_name = location.get_name ();
            }
            return (string) city_name;
        }
    }

    public string? state_name {
        owned get {
            GWeather.Location? parent = location.get_parent ();

            if (parent != null) {
                if (((GWeather.Location) parent).get_level () == ADM1) {
                    return ((GWeather.Location) parent).get_name ();
                }
            }

            return null;
        }
    }

    public string? country_name {
        owned get {
            return location.get_country_name ();
        }
    }

    public bool is_daytime {
         get {
            if (weather_info != null) {
                return ((GWeather.Info) weather_info).is_daytime ();
            }
            return true;
        }
    }

    public string sunrise_label {
        owned get {
            if (weather_info == null) {
                return "-";
            }

            ulong sunrise;
            if (!((GWeather.Info) weather_info).get_value_sunrise (out sunrise)) {
                return "-";
            }

            if (time_zone == null) {
                return "-";
            }

            var sunrise_time = new GLib.DateTime.from_unix_local (sunrise);
            sunrise_time = sunrise_time.to_timezone ((TimeZone) time_zone);
            return Utils.WallClock.get_default ().format_time (sunrise_time);
        }
    }

    public string sunset_label {
        owned get {
            if (weather_info == null) {
                return "-";
            }

            ulong sunset;
            if (!((GWeather.Info) weather_info).get_value_sunset (out sunset)) {
                return "-";
            }

            if (time_zone == null) {
                return "-";
            }

            var sunset_time = new GLib.DateTime.from_unix_local (sunset);
            sunset_time = sunset_time.to_timezone ((TimeZone) time_zone);
            return Utils.WallClock.get_default ().format_time (sunset_time);
        }
    }

    public string time_label {
        owned get {
            return Utils.WallClock.get_default ().format_time (date_time);
        }
    }

    public string? day_label {
        get {
            var d = date_time.get_day_of_year ();
            var t = local_time.get_day_of_year ();

            if (d < t) {
                // If it is Jan 1st there, and not Jan 2nd here, then it must be
                // Dec 31st here, so return "tomorrow"
                return (d == 1 && t != 2) ? _("Tomorrow") : _("Yesterday");
            } else if (d > t) {
                // If it is Jan 1st here, and not Jan 2nd there, then it must be
                // Dec 31st there, so return "yesterday"
                return (t == 1 && d != 2) ? _("Yesterday") : _("Tomorrow");
            } else {
                return null;
            }
        }
    }

    public TimeSpan local_offset {
        get {
            return local_time.get_utc_offset () - date_time.get_utc_offset ();
        }
    }

    private bool is_current (DateTime? sunrise, DateTime? sunset) {
        if (sunrise == null || sunset == null) {
            return false;
        }

        return (date_time.compare ((DateTime) sunrise) > 0) &&
                        (date_time.compare ((DateTime) sunset) < 0);
    }

    // CSS class for the current time of day
    public string state_class {
        get {
            if (sun_rise == null || sun_set == null) {
                return "none";
            }

            if (is_current (sun_rise, sun_set)) {
                return "day";
            }

            if (is_current (civil_rise, civil_set)) {
                return "civil";
            }

            return "night";
        }
    }

    private string _name;
    private GLib.TimeZone? time_zone;
    private GLib.DateTime local_time;
    private GLib.DateTime date_time;
    private GWeather.Info? weather_info;

    // When sunrise/sunset happens, at different corrections, in locations
    // timezone for calculating the colour pill
    private DateTime? sun_rise;
    private DateTime? sun_set;
    private DateTime? civil_rise;
    private DateTime? civil_set;
    // When we last calculated
    private int last_calc_day = -1;

    public WorldItem (GWeather.Location location) {
        Object (location: location);

        time_zone = location.get_timezone ();

        tick ();
    }

    private void calculate_riseset_at_correction (double latitude,
                                                  double longitude,
                                                  int year,
                                                  int month,
                                                  int day,
                                                  double correction,
                                                  out DateTime? sunrise,
                                                  out DateTime? sunset) requires (time_zone != null) {
        int rise_hour, rise_min;
        int set_hour, set_min;

        if (!calculate_sunrise_sunset (latitude,
                                       longitude,
                                       year,
                                       month,
                                       day,
                                       correction,
                                       out rise_hour,
                                       out rise_min,
                                       out set_hour,
                                       out set_min)) {
            sunrise = null;
            sunset = null;
            debug ("Location (%f,%f) has incalculable sunset/sunrise",
                   latitude,
                   longitude);
            return;
        }

        var utc_sunrise = (DateTime?) new DateTime.utc (year, month, day, rise_hour, rise_min, 0);
        if (utc_sunrise != null) {
            sunrise = ((DateTime) utc_sunrise).to_timezone ((TimeZone) time_zone);
        } else {
            sunrise = null;
            warning ("Sunrise for (%f,%f) resulted in %04i-%02i-%02i %02i:%02i",
                     latitude,
                     longitude,
                     year,
                     month,
                     day,
                     rise_hour,
                     rise_min);
        }

        var utc_sunset = (DateTime?) new DateTime.utc (year, month, day, set_hour, set_min, 0);
        if (utc_sunset != null && sunrise != null) {
            var local_sunset = ((DateTime) utc_sunset).to_timezone ((TimeZone) time_zone);
            if (local_sunset.compare ((DateTime) sunrise) < 0) {
                sunset = local_sunset.add_days (1);
            } else {
                sunset = local_sunset;
            }
        } else {
            sunset = null;
            warning ("Sunset for (%f,%f) resulted in %04i-%02i-%02i %02i:%02i",
                     latitude,
                     longitude,
                     year,
                     month,
                     day,
                     rise_hour,
                     rise_min);
        }
    }

    private void calculate_riseset () {
        // Where we are calculating for
        double latitude, longitude;
        // The current UTC day
        int year, month, day;

        if (date_time.get_day_of_year () == last_calc_day) {
            return;
        }

        if (!location.has_coords ()) {
            return;
        }

        location.get_coords (out latitude, out longitude);

        // Some locations, such as UTC, aren't actual locations and don't have
        // proper coords
        if (!latitude.is_finite () || !longitude.is_finite ()) {
            return;
        }

        var utc = date_time.to_utc ();
        utc.get_ymd (out year, out month, out day);

        calculate_riseset_at_correction (latitude,
                                         longitude,
                                         year,
                                         month,
                                         day,
                                         RISESET_CORRECTION_NONE,
                                         out sun_rise,
                                         out sun_set);
        calculate_riseset_at_correction (latitude,
                                         longitude,
                                         year,
                                         month,
                                         day,
                                         RISESET_CORRECTION_CIVIL,
                                         out civil_rise,
                                         out civil_set);

        last_calc_day = date_time.get_day_of_year ();
    }

    [Signal (run = "first")]
    public virtual signal void tick () {
        var wallclock = Utils.WallClock.get_default ();
        local_time = wallclock.date_time;

        if (time_zone == null) {
            return;
        }

        date_time = local_time.to_timezone ((TimeZone) time_zone);

        calculate_riseset ();

        // We don't use the normal constructor since we only want static data
        // and we do not want update() to be called.
        if (location.has_coords ()) {
            weather_info = (GWeather.Info) Object.new (typeof (GWeather.Info),
                                                       location: location,
                                                       enabled_providers: GWeather.Provider.NONE);
        }
    }

    public void serialize (GLib.VariantBuilder builder) {
        if (!automatic) {
            builder.open (new GLib.VariantType ("a{sv}"));
            builder.add ("{sv}", "location", location.serialize ());
            builder.close ();
        }
    }

    public static WorldItem? deserialize (Variant location_variant) {
        GWeather.Location? location = null;
        string key;
        Variant val;
        var world = GWeather.Location.get_world ();

        if (world == null) {
            return null;
        }

        var iter = location_variant.iterator ();
        while (iter.next ("{sv}", out key, out val)) {
            if (key == "location") {
                location = ((GWeather.Location) world).deserialize (val);
            }
        }

        if (location == null) {
            return null;
        } else if (((GWeather.Location) location).get_timezone_str () == null) {
            warning ("Invalid location “%s” – timezone unknown. Ignoring.",
                     ((GWeather.Location) location).get_name ());
            return null;
        } else {
            return new WorldItem ((GWeather.Location) location);
        }
    }
}