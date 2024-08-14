use super::DateTime;
use crate::{SimpleTimeZone, TimeZone, MONTH_SHORT_NAMES, WEEKDAY_SHORT_NAMES};

/// Displays a [`DateTime`] using short names and ISO 8601 time zones
pub struct DateTimeShortDisplay<'a, T: TimeZone = SimpleTimeZone> {
    date_time: &'a DateTime<T>,
    _24_hour: bool,
    weekday: bool,
}

impl<'a, T: TimeZone> DateTimeShortDisplay<'a, T> {
    /// Creates a new [`DateTimeFullDisplay`]
    pub(super) const fn new(date_time: &'a DateTime<T>) -> Self {
        DateTimeShortDisplay {
            date_time,
            _24_hour: false,
            weekday: true,
        }
    }

    /// Set to display using 24 hour time
    pub fn _24_hour(mut self) -> Self {
        self._24_hour = true;
        self
    }

    /// Set to display using 12 hour time
    pub fn _12_hour(mut self) -> Self {
        self._24_hour = false;
        self
    }

    /// Show the weekday
    pub fn show_weekday(mut self) -> Self {
        self.weekday = true;
        self
    }

    /// Don't show the weekday
    pub fn hide_weekday(mut self) -> Self {
        self.weekday = false;
        self
    }
}

impl<'a, T: TimeZone> core::fmt::Display for DateTimeShortDisplay<'a, T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.weekday {
            write!(
                f,
                "{}, ",
                WEEKDAY_SHORT_NAMES[self.date_time.day_of_week() as usize]
            )?;
        }

        write!(
            f,
            "{} {}, {} ",
            MONTH_SHORT_NAMES[self.date_time.month as usize],
            self.date_time.day,
            self.date_time.year
        )?;

        if self._24_hour {
            write!(
                f,
                "{:02}:{:02}:{:02}.{:03} ",
                self.date_time.hour,
                self.date_time.minute,
                self.date_time.second,
                self.date_time.microsecond
            )?;
        } else {
            let pm = self.date_time.hour >= 12;
            let hour = if self.date_time.hour == 0 {
                12
            } else if pm && self.date_time.hour != 12 {
                self.date_time.hour - 12
            } else {
                self.date_time.hour
            };

            write!(
                f,
                "{}:{:02}:{:02}.{:03} {} ",
                hour,
                self.date_time.minute,
                self.date_time.second,
                self.date_time.microsecond,
                if pm { "PM" } else { "AM" }
            )?;
        }

        self.date_time.time_zone.iso8601().fmt(f)
    }
}
