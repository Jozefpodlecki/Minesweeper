use std::hint::unreachable_unchecked;

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

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
