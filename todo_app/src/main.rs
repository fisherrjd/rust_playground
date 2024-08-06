mod task;

use task::{Task, Status, TaskList};

fn main() {

    let mut main = TaskList::new(1, String::from("TODO"));
    let task1 = Task{
        id:1,
        desc: String::from("implement login functionality"),
        status: Status::Open
    };
    let task2 = Task{
        id:2,
        desc: String::from("Build home page"),
        status: Status::Inprogress
    };   
    let task3 = Task{
        id:3,
        desc: String::from("Create user profile page"),
        status: Status::Complete
    };   

    main.add_task(task1);
    main.add_task(task2);
    main.add_task(task3);
    println!("{}", main);
}
