use crate::logical_clock::{LogicalClock, LogicalClockError};

struct VectorClock
{
    curr_node_id: usize,
    time_stamp: Vec<u8>
}

impl VectorClock {
    pub fn new(node_id: usize, num_nodes_in_system: usize) -> Self {
        VectorClock { curr_node_id: node_id, time_stamp: vec![0; num_nodes_in_system] }
    }
}

impl LogicalClock<Vec<u8>> for VectorClock {
    fn tick(&mut self) {
        let t = self.time_stamp.get_mut(self.curr_node_id).unwrap();
        *t += 1;
    }

    fn get_current_timestam(&self) -> &Vec<u8> {
        &self.time_stamp
    }

    fn compare_time_stamp(&self, received_ts: &Vec<u8>) -> Result<i8, LogicalClockError> {
        // Check if at least one node's time stamp stored localy is greater than the corresponding node time stamp received in the message 
        let cmp1 = self.time_stamp.iter().any(|t1| received_ts.iter().any(|t2| t2 < t1));
        
        // Check if at least one node's time stamp received in the message is greater than the corresponding node time stamp stored locally
        let cmp2 = received_ts.iter().any(|t1| self.time_stamp.iter().any(|t2| t2 < t1));

        if cmp1 && cmp2
        {
            Err(LogicalClockError::ConcurrentTimeStamps)
        } else {
            let a = if cmp1 {
                1
            } else {
                0
            };

            let b = if cmp2 {
                1
            } else {
                0
            };
            Ok(a - b)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vector_clock::VectorClock;
    use crate::logical_clock::{LogicalClock, LogicalClockError};

    #[test]
    fn compares_smaller_timestamp() {
        let mut vc = VectorClock::new(0, 3);

        vc.tick();
        vc.tick();

        let external_ts = vec![0,0,0];
        let res = vc.compare_time_stamp(&external_ts).unwrap();
        assert_eq!(res, 1)

    }

    #[test]
    fn compares_equal_timestamp() {
        let vc = VectorClock::new(0, 3);

        let external_ts = vec![0,0,0];
        let res = vc.compare_time_stamp(&external_ts).unwrap();
        assert_eq!(res, 0)

    }

    #[test]
    fn compares_greater_timestamp() {
        let vc = VectorClock::new(0, 3);

        let external_ts = vec![0,1,1];
        let res = vc.compare_time_stamp(&external_ts).unwrap();
        assert_eq!(res, -1)

    }

    #[test]
    fn compares_concurrent_timestamp() {
        let mut vc = VectorClock::new(0, 3);

        vc.tick();
        vc.tick();

        let external_ts = vec![0,0,1];
        let res = vc.compare_time_stamp(&external_ts);
        assert!(matches!(res, Err(LogicalClockError::ConcurrentTimeStamps)))
    }
}