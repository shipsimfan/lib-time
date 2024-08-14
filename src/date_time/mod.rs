use crate::{
    day_of_week, day_of_year, is_leap_year, month_length, week_of_year, SimpleTimeZone, TimeZone,
};
use core::num::{NonZeroI32, NonZeroU8};

mod from_timestamp;
mod full;
mod into_timestamp;
mod iso8601;
mod short;

#[cfg(target_os = "windows")]
mod windows;

pub use full::DateTimeFullDisplay;
pub use iso8601::DateTimeISO8601Display;
pub use short::DateTimeShortDisplay;

#[cfg(all(target_os = "windows", feature = "now"))]
use windows::get_current_date_time_utc;

#[cfg(all(target_os = "windows", feature = "now", feature = "local"))]
use windows::get_current_date_time_local;

/// A point in time described by its date and time
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DateTime<T: TimeZone = SimpleTimeZone> {
    /// The year, positive values are in the common era (CE), negative values are before the common
    /// era (BCE)
    year: NonZeroI32,

    /// The month, 0-indexed such that January is 0 and December is 11
    month: u8,

    /// The day of the month, 1-indexed
    day: NonZeroU8,

    /// The hour of the day, from 0-23
    hour: u8,

    /// The minute of the hour, from 0-59
    minute: u8,

    /// The second of the minute, from 0-59
    second: u8,

    /// The millisecond, from 0-999
    millisecond: u16,

    /// The microsecond, from 0-999
    microsecond: u16,

    /// The nanosecond, from 0-999
    nanosecond: u16,

    /// The time zone
    time_zone: T,
}

impl<T: TimeZone> DateTime<T> {
    /// Creates a new [`DateTime`], verifying all the values
    pub const fn new(
        year: NonZeroI32,
        month: u8,
        day: NonZeroU8,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        microsecond: u16,
        nanosecond: u16,
        time_zone: T,
    ) -> Self {
        assert!(month < 12);
        assert!(day.get() <= month_length(year, month).get());
        assert!(hour < 24);
        assert!(minute < 60);
        assert!(second < 60);
        assert!(millisecond < 1000);
        assert!(microsecond < 1000);
        assert!(nanosecond < 1000);

        unsafe {
            DateTime::new_unchecked(
                year,
                month,
                day,
                hour,
                minute,
                second,
                millisecond,
                microsecond,
                nanosecond,
                time_zone,
            )
        }
    }

    /// Creates a new [`DateTime`] without checking the validity of the values
    pub const unsafe fn new_unchecked(
        year: NonZeroI32,
        month: u8,
        day: NonZeroU8,
        hour: u8,
        minute: u8,
        second: u8,
        millisecond: u16,
        microsecond: u16,
        nanosecond: u16,
        time_zone: T,
    ) -> Self {
        DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
            millisecond,
            microsecond,
            nanosecond,
            time_zone,
        }
    }

    /// Creates a [`DateTime`] for the current time in UTC
    #[cfg(feature = "now")]
    pub fn now() -> Self {
        get_current_date_time_utc()
    }

    /// Creates a [`DateTime`] for the current time in local time
    #[cfg(all(feature = "now", feature = "local"))]
    pub fn now_local() -> Self {
        get_current_date_time_local()
    }

    /// Gets the year, positive values are in the common era (CE), negative values are before the
    /// common era (BCE)
    pub const fn year(&self) -> NonZeroI32 {
        self.year
    }

    /// Gets the month, 0-indexed such that January is 0 and December is 11
    pub const fn month(&self) -> u8 {
        self.month
    }

    /// Gets the day of the month, 1-indexed
    pub const fn day(&self) -> NonZeroU8 {
        self.day
    }

    /// Gets the hour of the day, from 0-23
    pub const fn hour(&self) -> u8 {
        self.hour
    }

    /// Gets the minute of the hour, from 0-59
    pub const fn minute(&self) -> u8 {
        self.minute
    }

    /// Gets the second of the minute, from 0-59
    pub const fn second(&self) -> u8 {
        self.second
    }

    /// Gets the millisecond, from 0-999
    pub const fn millisecond(&self) -> u16 {
        self.millisecond
    }

    /// Gets the microsecond, from 0-999
    pub const fn microsecond(&self) -> u16 {
        self.microsecond
    }

    /// Gets the nanosecond, from 0-999
    pub const fn nanosecond(&self) -> u16 {
        self.nanosecond
    }

    /// Gets the time zone
    pub const fn time_zone(&self) -> &T {
        &self.time_zone
    }

    /// Is the year of this [`DateTime`] a leap year?
    pub const fn is_leap_year(&self) -> bool {
        is_leap_year(self.year)
    }

    /// Gets the length of the month this [`DateTime`] is in
    pub const fn month_length(&self) -> NonZeroU8 {
        month_length(self.year, self.month)
    }

    /// Gets the day of the week this [`DateTime`] is on
    ///
    /// The weekday is 0-indexed, where 0 is Sunday and 6 is Saturday
    pub const fn day_of_week(&self) -> u8 {
        day_of_week(self.year, self.month, self.day)
    }

    /// Gets the day of the year this [`DateTime`] is on
    pub const fn day_of_year(&self) -> u16 {
        day_of_year(self.year, self.month, self.day)
    }

    /// Gets the week of the year this [`DateTime`] is on
    pub const fn week_of_year(&self) -> u8 {
        week_of_year(self.year, self.month, self.day)
    }

    /// Display this [`DateTime`] in ISO 8601 format
    pub const fn iso8601(&self) -> DateTimeISO8601Display<T> {
        DateTimeISO8601Display::new(self)
    }

    /// Display this [`DateTime`] using full names
    pub const fn full_display(&self) -> DateTimeFullDisplay<T> {
        DateTimeFullDisplay::new(self)
    }

    /// Display this [`DateTime`] using short names
    pub const fn short_display(&self) -> DateTimeShortDisplay<T> {
        DateTimeShortDisplay::new(self)
    }

    /// Sets the year
    pub fn set_year(&mut self, year: NonZeroI32) {
        assert!(self.day <= month_length(year, self.month));
        unsafe { self.set_year_unchecked(year) };
    }

    /// Sets the month
    pub fn set_month(&mut self, month: u8) {
        assert!(month < 12);
        assert!(self.day <= month_length(self.year, month));
        unsafe { self.set_month_unchecked(month) };
    }

    /// Sets the day
    pub fn set_day(&mut self, day: NonZeroU8) {
        assert!(day <= month_length(self.year, self.month));
        unsafe { self.set_day_unchecked(day) };
    }

    /// Sets the hour
    pub fn set_hour(&mut self, hour: u8) {
        assert!(hour < 24);
        unsafe { self.set_hour_unchecked(hour) };
    }

    /// Sets the minute
    pub fn set_minute(&mut self, minute: u8) {
        assert!(minute < 60);
        unsafe { self.set_minute_unchecked(minute) };
    }

    /// Sets the second
    pub fn set_second(&mut self, second: u8) {
        assert!(second < 60);
        unsafe { self.set_second_unchecked(second) };
    }

    /// Sets the millisecond
    pub fn set_millisecond(&mut self, millisecond: u16) {
        assert!(millisecond < 1000);
        unsafe { self.set_millisecond_unchecked(millisecond) };
    }

    /// Sets the microsecond
    pub fn set_microsecond(&mut self, microsecond: u16) {
        assert!(microsecond < 1000);
        unsafe { self.set_microsecond_unchecked(microsecond) };
    }

    /// Sets the nanosecond
    pub fn set_nanosecond(&mut self, nanosecond: u16) {
        assert!(nanosecond < 1000);
        unsafe { self.set_nanosecond_unchecked(nanosecond) };
    }

    /// Sets the year without checking the value
    pub unsafe fn set_year_unchecked(&mut self, year: NonZeroI32) {
        self.year = year;
    }

    /// Sets the month without checking the value
    pub unsafe fn set_month_unchecked(&mut self, month: u8) {
        self.month = month;
    }

    /// Sets the day without checking the value
    pub unsafe fn set_day_unchecked(&mut self, day: NonZeroU8) {
        self.day = day;
    }

    /// Sets the hour without checking the value
    pub unsafe fn set_hour_unchecked(&mut self, hour: u8) {
        self.hour = hour;
    }

    /// Sets the minute without checking the value
    pub unsafe fn set_minute_unchecked(&mut self, minute: u8) {
        self.minute = minute;
    }

    /// Sets the second without checking the value
    pub unsafe fn set_second_unchecked(&mut self, second: u8) {
        self.second = second;
    }

    /// Sets the millisecond without checking the value
    pub unsafe fn set_millisecond_unchecked(&mut self, millisecond: u16) {
        self.millisecond = millisecond;
    }

    /// Sets the microsecond without checking the value
    pub unsafe fn set_microsecond_unchecked(&mut self, microsecond: u16) {
        self.microsecond = microsecond;
    }

    /// Sets the nanosecond without checking the value
    pub unsafe fn set_nanosecond_unchecked(&mut self, nanosecond: u16) {
        self.nanosecond = nanosecond;
    }

    // TODO: strftime

    // TODO: From<TimeStamp>

    // TODO: now() and now_local()
}

impl<T: TimeZone> core::fmt::Display for DateTime<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.iso8601().fmt(f)
    }
}
