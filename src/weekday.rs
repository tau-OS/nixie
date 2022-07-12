use std::slice::Iter;

use chrono::Weekday;
use gettextrs::ngettext;

pub trait NixieWeekdays {
    fn symbol(&self) -> String;
    fn text(&self, count: u32) -> String;
    fn to_string(&self) -> String;
    fn into(&self) -> usize;
    fn iterator(&self) -> Iter<'static, Self>
    where
        Self: Sized;
}

impl NixieWeekdays for Weekday {
    fn symbol(&self) -> String {
        match self {
            Weekday::Mon => String::from("M"),
            Weekday::Tue => String::from("T"),
            Weekday::Wed => String::from("W"),
            Weekday::Thu => String::from("T"),
            Weekday::Fri => String::from("F"),
            Weekday::Sat => String::from("S"),
            Weekday::Sun => String::from("S"),
        }
    }

    fn text(&self, count: u32) -> String {
        match self {
            Weekday::Mon => ngettext("Monday", "Mondays", count),
            Weekday::Tue => ngettext("Tuesday", "Tuesdays", count),
            Weekday::Wed => ngettext("Wednesday", "Wednesdays", count),
            Weekday::Thu => ngettext("Thursday", "Thursdays", count),
            Weekday::Fri => ngettext("Friday", "Fridays", count),
            Weekday::Sat => ngettext("Saturday", "Saturdays", count),
            Weekday::Sun => ngettext("Sunday", "Sundays", count),
        }
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }

    fn into(&self) -> usize {
        static WEEKDAYS: [Weekday; 7] = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];

        WEEKDAYS
            .iter()
            .position(|&r| r == *self)
            .unwrap()
            .try_into()
            .unwrap()
    }

    fn iterator(&self) -> Iter<'static, Weekday>
    where
        Self: Sized,
    {
        static WEEKDAYS: [Weekday; 7] = [
            Weekday::Mon,
            Weekday::Tue,
            Weekday::Wed,
            Weekday::Thu,
            Weekday::Fri,
            Weekday::Sat,
            Weekday::Sun,
        ];
        WEEKDAYS.iter()
    }
}
