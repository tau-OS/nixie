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

public class ClockLocation : Object {
    public GWeather.Location loc { get; construct set; }
    public bool selected { get; set; }

    public ClockLocation (GWeather.Location loc, bool selected) {
        Object (loc: loc, selected: selected);
    }
}

public class LocationRow : Gtk.ListBoxRow {
    public ClockLocation data { get; construct set; }

    public string? lname { get; set; default = null; }
    public string? location { get; set; default = null; }
    public bool loc_selected { get; set; default = false; }

    public Gtk.Box main_box;

    public LocationRow (ClockLocation data) {
        Object (data: data);

        lname = data.loc.get_name ();
        location = data.loc.get_country_name ();
        data.bind_property ("selected", this, "loc-selected", SYNC_CREATE);

        var loc_label = new Gtk.Label (lname);
        loc_label.halign = Gtk.Align.START;
        loc_label.add_css_class ("cb-title");
        var loc_ct_label = new Gtk.Label (location);
        loc_ct_label.halign = Gtk.Align.START;
        loc_ct_label.add_css_class ("cb-subtitle");

        var loc_icon = new Gtk.Image.from_icon_name ("list-add-symbolic");
        loc_icon.halign = Gtk.Align.END;
        loc_icon.visible = data.selected ? true : false;

        var loc_box = new Gtk.Box (Gtk.Orientation.VERTICAL, 0);
        loc_box.append (loc_label);
        loc_box.append (loc_ct_label);
        loc_box.append (loc_icon);

        main_box = new Gtk.Box (Gtk.Orientation.HORIZONTAL, 0);
        main_box.add_css_class ("mini-content-block");
        main_box.append (loc_box);

        this.set_child (main_box);

        data.bind_property ("selected", this, "loc-selected", SYNC_CREATE);
    }
}

[GtkTemplate (ui = "/com/fyralabs/Nixie/worldlocationfinder.ui")]
public class Nixie.WorldLocationFinder : He.Window {
    private ListStore locations;
    private WorldFace world;
    private const int RESULT_COUNT_LIMIT = 4;

    private LocationRow? _selected_row = null;
    public LocationRow? selected_row {
        get {
            return _selected_row;
        } set {
            _selected_row = value;
        }
    }

    [GtkChild]
    unowned He.EmptyPage search_label;
    [GtkChild]
    unowned Gtk.Stack search_stack;
    [GtkChild]
    unowned Gtk.ListBox listbox;
    [GtkChild]
    unowned Gtk.SearchEntry search_entry;
    [GtkChild]
    unowned He.Button add_button;

    public WorldLocationFinder (Gtk.Window parent, WorldFace world_face) {
        Object (transient_for: parent);

        search_entry.set_key_capture_widget (this);

        world = world_face;

        add_button.sensitive = false;

        locations = new ListStore (typeof (ClockLocation));
        listbox.bind_model (locations, (data) => {
            return new LocationRow ((ClockLocation) data);
        });
    }

    construct {
        search_entry.search_changed.connect (() => {
            selected_row = null;

            // Remove old results
            locations.remove_all ();

            if (search_entry.text == "") {
                return;
            }

            string search = search_entry.text.normalize ().casefold ();
            var world = GWeather.Location.get_world ();
            if (world == null) {
                return;
            }

            query_locations ((GWeather.Location) world, search);

            if (locations.get_n_items () == 0) {
                return;
            }
            locations.sort ((a, b) => {
                var name_a = ((ClockLocation) a).loc.get_sort_name ();
                var name_b = ((ClockLocation) b).loc.get_sort_name ();
                return strcmp (name_a, name_b);
            });
            search_stack.visible_child_name = "results";
        });
        search_entry.notify["text"].connect (() => {
            if (search_entry.text == "")
                search_stack.visible_child_name = "empty";
        });
        search_label.action_button.visible = false;
    }

    public signal void location_added ();

    [GtkCallback]
    private void add_button_clicked () {
        location_added ();
        close ();
    }

    [GtkCallback]
    private void cancel_button_clicked () {
        close ();
    }

    private void query_locations (GWeather.Location lc, string search) {
        if (locations.get_n_items () >= RESULT_COUNT_LIMIT) return;

        switch (lc.get_level ()) {
            case CITY:
                var contains_name = lc.get_sort_name ().contains (search);

                var country_name = lc.get_country_name ();
                if (country_name != null) {
                    country_name = ((string) country_name).normalize ().casefold ();
                }
                var contains_country_name = country_name != null && ((string) country_name).contains (search);

                if (contains_name || contains_country_name) {
                    bool selected = location_exists (lc);
                    locations.append (new ClockLocation (lc, selected));
                }
                return;
            default:
                break;
        }

        var l = lc.next_child (null);
        while (l != null) {
            query_locations (l, search);
            if (locations.get_n_items () >= RESULT_COUNT_LIMIT) {
                return;
            }
            l = lc.next_child (l);
        }
    }

    public bool location_exists (GWeather.Location loc) {
        var exists = false;
        var n = locations.get_n_items ();
        for (int i = 0; i < n; i++) {
            var l = locations.get_object (i);
            if (l == loc) {
                exists = true;
                break;
            }
        }

        return exists;
    }

    public GWeather.Location? get_selected_location () {
        if (selected_row == null)
            return null;
        return ((LocationRow) selected_row).data.loc;
    }

    [GtkCallback]
    private void item_activated (Gtk.ListBoxRow listbox_row) {
        var row = (LocationRow) listbox_row;

        if (selected_row != null && selected_row != row) {
            ((LocationRow) selected_row).data.selected = false;
        }

        row.data.selected = !row.data.selected;
        if (row.data.selected) {
            selected_row = row;
        } else {
            selected_row = null;
        }

        add_button.sensitive = true;
    }
}
