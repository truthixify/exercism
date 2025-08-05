use std::ops::Rem;

use num_integer::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let hours_overflow_from_minutes = minutes.div_floor(&60);
        let minutes = minutes.mod_floor(&60);
        let hours = (hours + hours_overflow_from_minutes).mod_floor(&24);
        Self { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hours, self.minutes + minutes)
    }
}
