//! `wall-clock` provides a simple and very basic struct for repsenting time as one reads it off a
//! clock on the wall, e.g. with no concept of date, or time zone.
//!
//! ## Examples
//!
//! Making a wall clock time:
//!
//! ```rs
//! use wall_clock::WallClockTime;
//!
//! let wct = WallClockTime::new(15, 0, 0);
//! ```
//!
//! You can also use the `time!` macro to get a syntax resembling a literal:
//!
//! ```rs
//! use wall_clock::time;
//! let wct = time!(15:00:00);
//! ```
//!
//! ## Features
//!
//! `wall-clock` ships with the following features:
//!
//! - **diesel-pg**: Enables interop with PostgreSQL `TIME` columns using Diesel.
//! - **serde**: Enables serialization and deserialization with `serde`. _(Enabled by default.)_

use std::fmt::Debug;
use std::fmt::Display;
use std::str::FromStr;

#[cfg(feature = "diesel-pg")]
mod db;
#[cfg(feature = "serde")]
mod serde;

/// A representation of a time, as read from a wall clock, independent of date or time zone.
#[derive(Clone, Copy, Default, Eq, PartialEq, PartialOrd, Ord)]
#[cfg_attr(feature = "diesel-pg", derive(diesel::AsExpression, diesel::FromSqlRow))]
#[cfg_attr(feature = "diesel-pg", diesel(sql_type = ::diesel::sql_types::Time))]
pub struct WallClockTime {
  /// The number of seconds elapsed since midnight.
  seconds: u32,
  /// The number of microseconds elapsed since `seconds`.
  micros: u32,
}

impl WallClockTime {
  /// A new wall-clock time set to the provided hours, minutes, and seconds.
  ///
  /// ## Panic
  ///
  /// Panics if any values are too high for a wall clock (hours >= 24, minutes >= 60, seconds >=
  /// 60). Wall clocks don't know about leap seconds.
  pub const fn new(hours: u8, minutes: u8, seconds: u8) -> Self {
    Self::new_with_micros(hours, minutes, seconds, 0)
  }

  /// A new wall-clock time set to the provided hours, minutes, seconds, and microseconds.
  ///
  /// ## Panic
  ///
  /// Panics if any values are too high for a wall clock (hours >= 24, minutes >= 60, seconds >=
  /// 60). Wall clocks don't know about leap seconds.
  pub const fn new_with_micros(hours: u8, minutes: u8, seconds: u8, micros: u32) -> Self {
    assert!(hours < 24, "Hours out of bounds.");
    assert!(minutes < 60, "Minutes out of bounds.");
    assert!(seconds < 60, "Seconds out of bounds.");
    assert!(micros < 1_000_000, "Microseconds out of bounds.");
    Self { seconds: hours as u32 * 3_600 + minutes as u32 * 60 + seconds as u32, micros }
  }

  /// A new wall-clock time corresponding to the number of seconds and microseconds offset from
  /// midnight.
  ///
  /// ## Panic
  ///
  /// Panics if any values are higher than is valid for a wall clock (seconds >= 86,400; micros >=
  /// 1,000,000).
  pub const fn new_midnight_offset(seconds: u32, micros: u32) -> Self {
    assert!(seconds < 86_400, "Seconds out of bounds.");
    assert!(micros < 1_000_000, "Microseconds out of bounds.");
    Self { seconds, micros }
  }

  /// The number of hours since midnight.
  pub const fn hour(&self) -> u8 {
    (self.seconds / 3_600) as u8
  }

  /// The number of minutes since the last hour.
  pub const fn minute(&self) -> u8 {
    (self.seconds % 3600 / 60) as u8
  }

  /// The number of seconds since the last minute.
  pub const fn second(&self) -> u8 {
    (self.seconds % 60) as u8
  }

  /// The number of microseconds since the last second.
  pub const fn microsecond(&self) -> u32 {
    self.micros
  }
}

impl Debug for WallClockTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    Display::fmt(self, f)
  }
}

impl Display for WallClockTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:02}:{:02}:{:02}", self.hour(), self.minute(), self.second())?;
    if self.micros > 0 {
      write!(f, ".{:06}", self.micros)?;
    }
    Ok(())
  }
}

impl FromStr for WallClockTime {
  type Err = &'static str;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let seconds_micros: Vec<&str> = s.split('.').collect();
    if seconds_micros.len() > 2 {
      Err("Only one `.` allowed in wall-clock times")?;
    }
    let micros = match seconds_micros.get(1) {
      Some(micros) => micros.parse::<u32>().map_err(|_| "Invalid microseconds")?,
      None => 0,
    };
    let hms = seconds_micros.first().ok_or("Empty string")?;
    let hms: Vec<&str> = hms.split(':').collect();
    if hms.len() != 3 {
      Err("Invalid HH:MM:SS specified")?;
    }
    let hours = hms[0].parse::<u32>().map_err(|_| "Invalid HH")?;
    let minutes = hms[1].parse::<u32>().map_err(|_| "Invalid MM")?;
    let seconds = hms[2].parse::<u32>().map_err(|_| "Invalid SS")?;
    Ok(Self { seconds: hours * 3600 + minutes * 60 + seconds, micros })
  }
}

/// Construct a wall clock time from a `HH:MM:SS` literal.
///
/// ## Examples
///
/// ```
/// # use wall_clock::time;
/// let t = time!(15:30:45);
/// assert_eq!(t.hour(), 15);
/// assert_eq!(t.minute(), 30);
/// assert_eq!(t.second(), 45);
/// ```
#[macro_export]
macro_rules! time {
  ($h:literal:$m:literal:$s:literal) => {{
    #[allow(clippy::zero_prefixed_literal)]
    {
      $crate::WallClockTime::new($h, $m, $s)
    }
  }};
}

#[cfg(test)]
mod tests {
  use assert2::check;

  use crate::WallClockTime;

  #[test]
  fn test_hours() {
    check!(time!(09:30:00).hour() == 9);
    check!(time!(16:00:00).hour() == 16);
    check!(time!(17:15:30).hour() == 17);
  }

  #[test]
  fn test_minutes() {
    check!(time!(09:30:00).minute() == 30);
    check!(time!(16:00:00).minute() == 0);
    check!(time!(17:15:30).minute() == 15);
  }

  #[test]
  fn test_seconds() {
    check!(time!(16:00:00).second() == 0);
    check!(time!(17:15:30).second() == 30);
  }

  #[test]
  fn test_micros() {
    check!(WallClockTime::new_with_micros(9, 30, 0, 0).microsecond() == 0);
    check!(WallClockTime::new_with_micros(17, 15, 30, 600_000).microsecond() == 600_000);
  }

  #[test]
  fn test_display() {
    check!(time!(16:00:00).to_string() == "16:00:00");
    check!(format!("{:?}", time!(16:00:00)) == "16:00:00");
    check!(time!(17:15:30).to_string() == "17:15:30");
    check!(WallClockTime::new_with_micros(17, 15, 30, 600_000).to_string() == "17:15:30.600000");
  }

  #[test]
  fn test_parse() -> Result<(), &'static str> {
    check!("09:30:00".parse::<WallClockTime>()? == time!(09:30:00));
    check!("17:15:30".parse::<WallClockTime>()? == time!(17:15:30));
    check!(
      "18:30:01.345678".parse::<WallClockTime>()?
        == WallClockTime::new_with_micros(18, 30, 1, 345_678)
    );
    Ok(())
  }
}
