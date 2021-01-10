#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut delta = 0;
        if minutes % 60 < 0 {
            delta = -1;
        }
        Clock {
            hours: (((minutes / 60) % 24) + 24 + ((hours % 24) + 24) + delta) % 24,
            minutes: (60 + (minutes % 60)) % 60,
        }
    }

    pub fn add_minutes(&self, min: i32) -> Self {
        let mut delta = 0;
        if (self.minutes + min) % 60 < 0 {
            delta = -1;
        }
        Clock {
            hours: ((((self.minutes + min) / 60) % 24) + 24 + ((self.hours % 24) + 24) + delta) % 24,
            minutes: (60 + ((self.minutes + min) % 60)) % 60,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:#02}:{:#02}", self.hours, self.minutes)
    }
}

#[test]
#[ignore]
fn test_negative_hour_roll_over() {
    println!("{}", (-40 / 60));
}
