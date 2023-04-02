use crate::logical_clock::{LogicalClock, LogicalClockError};

struct VectorClock
{
    node_number: usize,
    time_stamp: Vec<u8>
}

impl VectorClock {
    pub fn new(node_num: usize, num_nodes_in_system: usize) -> Self {
        VectorClock { node_number: node_num, time_stamp: vec![0; num_nodes_in_system] }
    }
}

impl LogicalClock<Vec<u8>> for VectorClock {
    fn tick(&mut self) {
        let t = self.time_stamp.get_mut(self.node_number).unwrap();
        *t += 1;
    }

    fn get_current_timestam(&self) -> &Vec<u8> {
        &self.time_stamp
    }

    fn compare_time_stamp(&self, received_ts: &Vec<u8>) -> Result<u8, LogicalClockError> {
        let cmp1 = self.time_stamp.iter().any(|t1| received_ts.iter().any(|t2| t2 > t1));
        let cmp2 = received_ts.iter().any(|t1| self.time_stamp.iter().any(|t2| t2 > t1));

        if cmp1 && cmp2
        {
            Err(LogicalClockError::ConcurrentTimeStamps)
        } else {
            Ok(cmp1 as u8 - cmp2 as u8)
        }
    }
}

