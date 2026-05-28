use std::fmt;
use std::time;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut _hours: u64 = 0;
        let _minutes: u64;
        let mut _hours_offset: i32 = 0;

        // println!("input h: {}, m: {}", hours, minutes);
        // TODO handle negative hours and minutes
        if hours < 0 {
            _hours = hours.rem_euclid(24) as u64;
        } else {
            _hours = hours.abs() as u64;
        }

        if minutes < 0 {
            _hours_offset = (minutes as f32 / 60.0).floor().rem_euclid(24.0) as i32;
            // println!("_hours_offset {}", _hours_offset);
            _minutes = minutes.rem_euclid(60) as u64;
            // println!("_hours before {}", _hours);
            _hours = (_hours as i32 + _hours_offset).abs() as u64;
            // println!("_hours after {}", _hours);
        } else {
            _minutes = minutes.abs() as u64;
        }

        // println!("_hours: {}, _minutes: {}", _hours, _minutes);

        let _secs = (time::Duration::from_hours(_hours) + time::Duration::from_mins(_minutes as u64)).as_secs();

        let _hrs: i32 = ((_secs % 86400) / 3600).try_into().unwrap();
        let _mins: i32 = ((_secs % 3600) / 60).try_into().unwrap();

        // println!("calc s: {}, hrs: {}, mins: {}", _secs, _hrs, _mins);

        Clock{hours: _hrs, minutes: _mins}
   }

    pub fn add_minutes(&self, minutes: i32) -> Self {

        // println!("add_mins: {}", minutes);
        // println!("Current: {}", self);
        let _minutes: u64;
        let mut _hour_offset: i32 = 0;
        let mut _hours: u64 = 0;
        let mut _secs: u64 = 0;

        let mut _current_duration = time::Duration::from_hours(self.hours as u64) + time::Duration::from_mins(self.minutes as u64);
        let _duration = time::Duration::from_mins(minutes.abs().rem_euclid(1440) as u64);
        let _full_day = time::Duration::from_hours(24);
        let mut _secs: u64 = 0;
        // println!("cdur: {:?}, full: {:?}", _current_duration, _full_day);

        if (minutes < 0) & (_current_duration > _duration) {
            // println!("dur: -{:?}", _duration);
            _secs = _current_duration.as_secs() - _duration.as_secs();
        } else if (minutes < 0) & (_current_duration < _duration) {
            // println!("dur: -{:?} + full", _duration);
            _secs = (_current_duration.as_secs() + _full_day.as_secs()) - _duration.as_secs();
        } else {
            // println!("dur: {:?}", _duration);
            _secs = _current_duration.as_secs() + _duration.as_secs();
        }

        // println!("cur: {:?}", _current_duration);
        // let _secs = (time::Duration::from_hours(_hours) + _current_duration).as_secs();

        let _hrs: i32 = ((_secs % 86400) / 3600).try_into().unwrap();
        let _mins: i32 = ((_secs % 3600) / 60).try_into().unwrap();
        // println!("add_minutes s: {}, hrs: {}, mins: {}, dur: {:?}", _secs, _hrs, _mins, _current_duration);

        Clock { hours: _hrs, minutes: _mins }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
