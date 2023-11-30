pub struct LogEntry {
    pub method: String,
    pub username: String,
    pub resource: String,
}

pub trait Logging {
    fn lookup(&self) {
        todo!();
    }

    fn append(&self, time: String, method: String, username: String, resource: String);
}
