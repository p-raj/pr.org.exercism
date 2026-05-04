use std::fmt;
use std::cmp::Ordering;

pub struct Clock {
    hours: i32,
    minutes: i32,
}

const HH: i32 = 24;
const MM: i32 = 60;

impl Clock {
    fn wind_hours(hours: i32) -> (i32, i32) {
        let mut hh = hours;
        let mut wind = 0;
        while hh < 0 {
            hh += HH;
            wind += 1;
        }
        (hh, wind)
    }

    fn wind_minutes(minutes: i32) -> (i32, i32) {
        let mut mm = minutes;
        let mut wind = 0;
        while mm < MM {
            mm += MM;
            wind += 1;
        }
        (mm, wind)
    }

    fn mm_to_hhmm(minutes: i32) -> (i32, i32) {
        let mut mm = minutes;
        let mut hh = 0;
        while mm >= MM {
            mm -= MM;
            hh += 1;
        }
        (hh%HH, mm)
    }

    fn calc(hours: i32, minutes: i32) -> (i32, i32) {
        let (mm, wind) = Clock::wind_minutes(minutes);
        let (hh, _) = Clock::wind_hours(hours-wind);
        Clock::mm_to_hhmm(hh*MM + mm)
    }
    
    pub fn new(hours: i32, minutes: i32) -> Self {
        let (hours, minutes) = Clock::calc(hours, minutes);
        Clock {
            hours,
            minutes
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let (hours, minutes) = Clock::calc(self.hours, self.minutes + minutes);
        Clock {
            hours,
            minutes
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl Ord for Clock {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hours == other.hours && self.minutes == other.minutes {
            Ordering::Equal
        }
        else if (self.hours == other.hours && self.minutes > other.minutes) || (self.hours > other.hours) {
            Ordering::Greater
        }
        else {
            Ordering::Less
        }   
    }
}

impl PartialOrd for Clock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Clock {}
