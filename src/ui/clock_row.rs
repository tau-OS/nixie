use std::thread;
use std::time::Duration;

use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
use gtk::glib::{clone, MainContext, PRIORITY_DEFAULT};
use gtk::{self, glib};
use gweather::Location;
use he::prelude::*;
use he::{MiniContentBlock, TextButton};

pub fn generate_row(location: Location) -> MiniContentBlock {
    let tz: Tz = location.timezone().unwrap().identifier().parse().unwrap();
    let time = &tz.from_utc_datetime(&Utc::now().naive_utc());

    let time_label = &TextButton::new(
        &time
            .time()
            .format("%H:%M:%S %p") // 09:34:02 AM
            .to_string(),
    );

    let block = MiniContentBlock::builder()
        .title(&location.city_name().unwrap().to_string())
        .subtitle(
            &time
                .date()
                .format("%a, %h %d") // Sun, Jun 19
                .to_string(),
        )
        .primary_button(time_label)
        .build();

    let (sender, receiver) = MainContext::channel(PRIORITY_DEFAULT);

    thread::spawn(move || loop {
        sender.send(true).expect("Could not send through channel");
        thread::sleep(Duration::from_secs(1));
    });

    receiver.attach(
        None,
        clone!(@weak block => @default-return Continue(false),
        move |_not_used| {
            block.set_primary_button(&TextButton::new(
                &tz.from_utc_datetime(&Utc::now().naive_utc())
                    .time()
                    .format("%H:%M:%S %p") // 09:34:02 AM
                    .to_string(),
            ));

            Continue(true)
        }),
    );

    return block;
}
