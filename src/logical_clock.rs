pub enum LogicalClockError {
    ConcurrentTimeStamps,
    // Undefined {
    //     message: String
    // }
}


pub trait LogicalClock<T> {
    /// increment local timestamp
    fn tick(&mut self);
    /// Returns the current state of local time stamp
    fn get_current_timestam(&self) -> &T;
    /// Return error if timestamps are concurrent.
    fn compare_time_stamp(&self, time_stamp: &T) -> Result<u8, LogicalClockError>;
}