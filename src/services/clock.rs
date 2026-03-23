use chrono::{DateTime, Utc};

pub trait SystemClock {
    fn utc_now(&self) -> DateTime<Utc>;
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DefaultSystemClock;

impl SystemClock for DefaultSystemClock {
    fn utc_now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}
