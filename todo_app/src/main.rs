mod task, task_list;

use task::{Task, Status};
use task_list::{Task_list};

fn main() {
    let _task1 = Task{
        id:1,
        desc: String::from("implement login functionality"),
        status: Status::Open
    };
    let _task2 = Task{
        id:2,
        desc: String::from("Build home page"),
        status: Status::Inprogress
    };   
    let _task3 = Task{
        id:3,
        desc: String::from("Create user profile page"),
        status: Status::Complete
    };   

    let mut tasks: Vec<Task> = Vec::new();
}
