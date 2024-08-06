mod task;

use task::{Task, Status};

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

    println!("{}", _task1);
    println!("{}", _task2);
    println!("{}", _task3);
}
