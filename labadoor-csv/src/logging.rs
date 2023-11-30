use crate::CSV;
use labadoor_logging::{LogEntry, Logging};

impl Logging for CSV {
    fn append(&self, time: String, method: String, username: String, resource: String) {
        println!("{},{},{},{}", time, method, username, resource);
    }
}
