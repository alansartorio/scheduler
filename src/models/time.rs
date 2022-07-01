use std::fmt::{Debug, Display};
use std::ops::Sub;
use std::{error::Error, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Time {
    pub hour: u8,
    pub minutes: u8,
}

impl Time {
    pub fn new(hour: u8, minutes: u8) -> Time {
        assert!(hour < 24 || (hour == 24 && minutes == 0));
        assert!(minutes < 60);
        Time { hour, minutes }
    }
}

impl Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Time: \"{}\"", self)
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minutes)
    }
}


impl FromStr for Time {
    type Err = Box<dyn Error>;
    fn from_str(s: &str) -> Result<Time, Self::Err> {
        let (hour, minutes) = s.split_once(':').ok_or("Time is missing ':'")?;
        Ok(Time {
            hour: hour.parse()?,
            minutes: minutes.parse()?,
        })
    }
}

impl Sub for Time {
    type Output = u64;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.hour as u64 * 60 + self.minutes as u64) - (rhs.hour as u64 * 60 + rhs.minutes as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_time() {
        let time = "03:40".parse::<Time>().unwrap();
        assert_eq!(time.hour, 3);
        assert_eq!(time.minutes, 40);
        assert_eq!(
            time,
            Time {
                hour: 3,
                minutes: 40
            }
        );
    }

    #[test]
    fn time_ordering() {
        assert!(Time::new(3, 40) < Time::new(3, 41));
        assert!(Time::new(3, 40) < Time::new(4, 40));
    }
}
