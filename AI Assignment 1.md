# Rust Programming Assignment: Command-line TODO App

## Objective
Develop a command-line TODO application using Rust. This project will reinforce your understanding of Rust's core concepts, including structs, enums, file I/O, and error handling.

## Requirements

### 1. Task Management
- [ ] Implement a `Task` struct with fields for id, description, and completion status
- [ ] Create functions to:
  - Add new tasks
  - List all tasks
  - Mark tasks as complete
  - Remove tasks

### 2. Data Persistence
- [ ] Save tasks to a JSON file
- [ ] Load tasks from the JSON file on application startup

### 3. Command-line Interface
- [ ] Use the `clap` crate to parse command-line arguments
- [ ] Implement the following subcommands:
  - `add`: Add a new task
  - `list`: Display all tasks
  - `complete`: Mark a task as complete
  - `remove`: Delete a task

### 4. Error Handling
- [ ] Utilize `Result` and `Option` types for robust error handling
- [ ] Display user-friendly error messages

### 5. Testing
- [ ] Write unit tests for core functionality
- [ ] Create at least one integration test for the CLI

## Bonus Challenges
- [ ] Implement due dates for tasks
- [ ] Add priority levels (e.g., High, Medium, Low)
- [ ] Create a simple text-based UI using the `cursive` crate

## Submission Guidelines
1. Initialize a new Rust project using Cargo
2. Implement the required features
3. Include a README.md with:
   - Project description
   - Instructions for building and running the application
   - Examples of usage
4. Push your code to a GitHub repository
5. Submit the link to your GitHub repository

## Grading Criteria
- Correct implementation of required features (60%)
- Code quality and organization (20%)
- Error handling and user experience (10%)
- Testing coverage (10%)

## Resources
- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)
- [clap Documentation](https://docs.rs/clap/)
- [serde Documentation](https://serde.rs/)

Good luck, and happy coding!