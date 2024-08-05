use std::fmt;

#[derive(Debug)]
pub enum Status{
    Open,
    Inprogress,
    Complete
}
pub struct Task {
    pub id: u32,
    pub desc: String,
    pub status: Status,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task {}: {} - {:?}", self.id, self.desc, self.status)
    }
}