use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    year: u16,
    month: u8,
    day: u8,
}
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl FromStr for Date {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut ymd = s.split('-');
        let year = ymd.next().ok_or(())?.parse().map_err(|_| ())?;
        let month = ymd.next().ok_or(())?.parse().map_err(|_| ())?;
        let day = ymd.next().ok_or(())?.parse().map_err(|_| ())?;

        if year != 1518 || month > 12 || day > 31 {
            return Err(());
        }

        if ymd.next().is_none() {
            Ok(Self { year, month, day })
        } else {
            Err(())
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl FromStr for Time {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut hm = s.split(':');
        let hour = hm.next().ok_or(())?.parse().map_err(|_| ())?;
        let minute = hm.next().ok_or(())?.parse().map_err(|_| ())?;

        if hour >= 24 || minute >= 60 {
            return Err(());
        }

        if hm.next().is_none() {
            Ok(Self { hour, minute })
        } else {
            Err(())
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DateTime {
    pub date: Date,
    pub time: Time,
}

impl fmt::Display for DateTime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.date, self.time)
    }
}

impl FromStr for DateTime {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dt = s.split(' ');
        let date = dt.next().ok_or(())?.parse().map_err(|_| ())?;
        let time = dt.next().ok_or(())?.parse().map_err(|_| ())?;

        if dt.next().is_none() {
            Ok(Self { date, time })
        } else {
            Err(())
        }
    }
}
