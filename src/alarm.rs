use std::cell::{Cell, RefCell};

use chrono::{DateTime, Datelike, Local, Weekday};

#[derive(PartialEq, Eq, Clone)]
enum State {
    Ready,
    Ringing,
    _Snoozing,
}

#[derive(Clone)]
pub struct Alarm {
    pub id: RefCell<String>,
    pub name: RefCell<String>,
    pub time: Cell<DateTime<Local>>,
    pub days: Vec<Weekday>,
    pub ring: Cell<bool>,
    pub active: Cell<bool>,

    state: State,
}

impl Default for Alarm {
    fn default() -> Self {
        Self {
            id: RefCell::new(String::from("")),
            name: RefCell::new(String::from("")),
            time: Cell::new(Local::now()),
            days: Vec::new(),
            ring: Cell::new(true),
            active: Cell::new(false),
            state: State::Ready,
        }
    }
}

impl Alarm {
    fn update_alarm_time(&self) {
        if self.time.get().timestamp() < Local::now().timestamp() {
            // Alarm has passed, find new date
            let mut i = 1;
            let mut day = self.time.get().with_day(self.time.get().day() + i).unwrap();

            while self.days.iter().any(|p| *p != day.weekday()) {
                day = self.time.get().with_day(self.time.get().day() + i).unwrap();
                i = i + 1;
            }

            self.time.replace(day);
        }
    }

    fn reset(&mut self) {
        self.update_alarm_time();
        self.state = State::Ready;
    }

    pub fn stop(&mut self) {
        self.state = State::Ready;
    }

    pub fn set_active(&mut self, active: bool) {
        if active != self.active.get() {
            self.active.replace(active);
            if active {
                self.reset();
            } else if self.state == State::Ringing {
                self.stop();
            }
        }
    }
}
