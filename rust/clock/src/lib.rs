use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    t: u32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let h = (self.t / 60) % 24;
        let m = self.t % 60;
        write!(f, "{:02}:{:02}", h, m)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let t = normalize(hours * 60 + minutes);
        Clock { t: t }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let t = normalize(self.t as i32 + minutes);
        Clock { t: t }
    }
}

fn normalize(t: i32) -> u32 {
    let h = (t.div_euclid(60)).rem_euclid(24);
    let m = t.rem_euclid(60);
    (h * 60 + m) as u32
}
