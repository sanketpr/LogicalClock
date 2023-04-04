use std::cmp::max;
use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::event::{Event, EventType};
use crate::logical_clock::{LogicalClock, LogicalClockError};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LamportClock {
    pub time_stamp: u32,
}

impl LamportClock {
    pub fn new() -> Self {
        LamportClock {
            time_stamp: 0
        }
    }
}

impl LogicalClock<u32> for LamportClock {
    fn compare_time_stamp(&self, _time_stamp: &u32) -> Result<i8, LogicalClockError> {
        todo!()
    }

    fn tick(&mut self) {
        self.time_stamp += 1;
    }

    fn get_current_timestam(&self) -> &u32 {
        &self.time_stamp
    }
}
