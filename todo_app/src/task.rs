use std::fmt;

/* 
----------------TODO----------------

* Status Enum

----------------TODO----------------
*/

#[derive(Debug)]
pub enum Status{
    Open,
    Inprogress,
    Complete
}

/* 
----------------TODO----------------

* Task Struct

----------------TODO----------------
*/

pub struct Task {
    pub id: u32,
    pub desc: String,
    pub status: Status
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task {}: {:?} - {}", self.id, self.status, self.desc)
    }
}

/* 
----------------TODO----------------

* Create List
* Add Tasks
* Remove Tasks
* Update Task Status - Remove task after complete? 

----------------TODO----------------
*/
    pub struct TaskList {
        id: u32,
        title: String,
        list: Vec<Task>
    }

    impl TaskList {
        pub fn new(id: u32, title: String) -> Self{
            TaskList{
                id,
                title,
                list: Vec::new(),
            }
        }

        pub fn add_task(&mut self, task: Task) {
            self.list.push(task);
        }
    }
