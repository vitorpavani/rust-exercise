// Instructions
// Implement a clock that handles times without dates.

// You should be able to add and subtract minutes to it.

// Two clocks that represent the same time should be equal to each other.

// You will also need to implement .to_string() for the Clock struct. We will be using this to display the Clock's state. You can either do it via implementing it directly or using the Display trait.

// Did you implement .to_string() for the Clock struct?

// If so, try implementing the Display trait for Clock instead.

// Traits allow for a common way to implement functionality for various types.

// For additional learning, consider how you might implement String::from for the Clock type. You don't have to actually implement this—it's redundant with Display, which is generally the better choice when the destination type is String—but it's useful to have a few type-conversion traits in your toolkit.

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
      Clock {
        hours: ()
      }
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", self.hours, self.minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut hours = self.hours;
        let mut minutes = self.minutes + minutes;

        if minutes >= 60 {
            hours += minutes / 60;
            minutes %= 60;
        }

        Clock::new(hours, minutes)
    }
}
