#[allow(unused_imports)]
use std::cmp::max;
use serde::{Serialize, Deserialize};
use crate::event::{Event, EventType};

pub trait Clock {
    fn process_event(&mut self, event: Event) -> u32;
    fn get_current_timestam(&self) -> u32;
    fn new() -> Self;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LamportClock {
    pub time_stamp: u32,
}

impl Clock for LamportClock {
    fn new() -> Self {
        LamportClock {
            time_stamp: 0
        }
    }

    fn process_event(&mut self, event: Event) -> u32 {
        match event.r#type {
            EventType::Local => {
                self.time_stamp += 1;
            },
            EventType::Recieve => {
                // TODO: Log error if event data is None
                let message = event.message.unwrap();
                let sender_id = &message.sender_id;
                self.time_stamp = max(self.time_stamp, message.time_stamp) + 1;
                println!("{} Recieved a Message. Curr time {}", sender_id, self.time_stamp);
            },
            EventType::Send => {
                // TODO: Make sure the order of appending timestamp to Message and incrementing 
                // local timestamp is correct
                let sender_id =  &event.clone().message.unwrap().sender_id;
                if let Some(mut message) = event.message {
                    message.time_stamp = self.time_stamp;
                }
                self.time_stamp += 1;
                println!("{} Sent a Message. Curr time {}", sender_id, self.time_stamp);
            }
        };
        self.time_stamp
    }

    fn get_current_timestam(&self) -> u32 {
        self.time_stamp
    }
}
