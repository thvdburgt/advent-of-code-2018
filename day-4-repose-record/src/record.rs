use std::fmt;
use std::str::FromStr;

use datetime::*;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum RecordType {
    ShiftStart(u32),
    FallsAsleep,
    WakesUp,
}

impl fmt::Display for RecordType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RecordType::ShiftStart(id) => write!(f, "Guard #{} begins shift", id),
            RecordType::FallsAsleep => write!(f, "falls asleep"),
            RecordType::WakesUp => write!(f, "wakes up"),
        }
    }
}

impl FromStr for RecordType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "falls asleep" {
            Ok(RecordType::FallsAsleep)
        } else if s == "wakes up" {
            Ok(RecordType::WakesUp)
        } else {
            let mut s = s.split_whitespace();
            if s.next().ok_or(())? != "Guard" {
                println!("no 'Guard'");
                return Err(());
            }
            let id = s.next().ok_or(())?;
            if !id.starts_with('#') {
                println!("id does not start with #: '{}'", id);
                return Err(());
            }
            let id = id[1..].parse().map_err(|_| ())?;
            if s.next().ok_or(())? != "begins" {
                println!("no 'begins'");
                return Err(());
            }
            if s.next().ok_or(())? != "shift" {
                println!("no 'shift'");
                return Err(());
            }

            Ok(RecordType::ShiftStart(id))
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Record {
    pub datetime: DateTime,
    pub record_type: RecordType,
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {}", self.datetime, self.record_type)
    }
}

impl FromStr for Record {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            return Err(());
        }
        let s = &s[1..];

        let mut iter = s.split(']');
        let datetime = iter.next().ok_or(())?.parse().map_err(|_| ())?;
        let record_type = iter.next().ok_or(())?.trim().parse().map_err(|_| ())?;

        Ok(Self {
            datetime,
            record_type,
        })
    }
}
