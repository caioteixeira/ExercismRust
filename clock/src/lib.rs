use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    minutes: i32,
}

const HOUR: i32 = 60;
const DAY: i32 = 24 * HOUR;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut total_minutes = hours * HOUR + minutes;
        if total_minutes < 0 {
            total_minutes %= DAY;
            total_minutes += DAY;
        }

        Clock {
            minutes: total_minutes % (DAY),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours(), self.minutes() + minutes)
    }

    pub fn minutes(&self) -> i32 {
        self.minutes % HOUR
    }

    pub fn hours(&self) -> i32 {
        self.minutes / HOUR
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{hours:0>2}:{minutes:0>2}",
            hours = self.hours(),
            minutes = self.minutes()
        )
    }
}
