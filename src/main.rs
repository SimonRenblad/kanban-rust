extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::DetectCursorPos;
use std::io::{Write, stdout, stdin};

enum Status {
    TODO,
    IN_PROGRESS,
    DONE,
}

enum Mode {
    Normal,
    Move,
    Edit,
    Create
}

struct Task {
    title: String,
    status: Status
}

// struct Board {
//     todo_tasks: Vec<Task>,
//     in_progress_tasks: Vec<Task>,
//     done_tasks: Vec<Task>,
// }
// 
// impl Board {
//     fn new() -> Self {
//         Self { todo_tasks: vec![], in_progress_tasks: vec![], done_tasks: vec![] }
//     }
// }

fn main() {
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    // define globals
    let mut cursor_x = 1;
    let mut cursor_y = 1;
    let mut mode: Mode = Mode::Normal;
    let mut input_vector: String = String::from("");

    let mut board: Vec<Task> = Vec::new();
    board.push(Task { title: String::from("send email"), status: Status::DONE });

    board.push(Task { title: String::from("wash dishes"), status: Status::DONE });
    board.push(Task { title: String::from("finish task"), status: Status::TODO });

    write!(stdout, "{}{}q to exit. Type stuff, use alt, and so on.",
           // Clear the screen.
           termion::clear::All,
           // Goto (1,1).
           termion::cursor::Goto(cursor_x, cursor_y));
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {

        // clear the screen
        write!(stdout, "{}{}",
               // Clear the screen.
               termion::clear::All,
               // Goto (1,1).
               termion::cursor::Goto(1, 1));
        stdout.flush().unwrap();
        // Print the key we type...
        match mode {
            Mode::Normal => {
                match c.unwrap() {
                    // Exit.
                    Key::Char('q') => break,
                    Key::Char('h') => {
                        cursor_x -= 1;
                        if cursor_x < 1 { cursor_x = 1 };
                        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y));
                     },
                    Key::Char('j') => {
                        cursor_y += 1;
                        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y));
                     },
                    Key::Char('k') => {
                        cursor_y -= 1;
                        if cursor_y < 1 { cursor_y = 1 };
                        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y));
                     },
                    Key::Char('l') => {
                        cursor_x += 1;
                        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y));
                     },
                    Key::Char('n') => {
                        mode = Mode::Create;
                        write!(stdout, "{}New task: ", termion::cursor::Goto(1, 15)); 
                    }
                    _              =>{},
                }
            }
            Mode::Edit => {
                // print edit area
                write!(stdout, "{}New task: {}", termion::cursor::Goto(1, 15), input_vector);
                stdout.flush().unwrap();
                match c.unwrap() {
                    Key::Esc => {
                        mode = Mode::Normal;
                        input_vector.clear();
                        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y));
                    }
                    // Key::Enter => {
                    //     // if the buffer is non-empty, set the task name as this
                    //     if !input_vector.is_empty() {

                    // }
                    // Key::Char(c) => {
                    //     // add to temp buffer vector
                    //     input_vector.push(c); 
                    // }
                    _ => {}
                }
            }
            Mode::Create => {
                stdout.flush().unwrap();
                match c.unwrap() {
                    Key::Esc => {
                        mode = Mode::Normal;
                        input_vector.clear();
                        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y));
                    }
                    Key::Char('\n') |  Key::Char('\r') => {
                        // if the buffer is non-empty, create new task and mark as to-do
                        if !input_vector.is_empty() {
                            board.push(Task { title: input_vector.clone(), status: Status::TODO });
                        }
                        input_vector.clear();
                        mode = Mode::Normal;
                        write!(stdout, "{}", termion::cursor::Goto(cursor_x, cursor_y));
                    }
                    Key::Backspace | Key::Delete => {
                        // if buffer is non-empty, pop 
                        input_vector.pop();
                        write!(stdout, "{}New task: {}", termion::cursor::Goto(1, 15), input_vector);
                    }
                    Key::Char(c) => {
                        input_vector.push(c); 
                        write!(stdout, "{}New task: {}", termion::cursor::Goto(1, 15), input_vector);
                    }
                    _ => {
                        write!(stdout, "{}New task: {}", termion::cursor::Goto(1, 15), input_vector);
                    }
                }
            }
            Mode::Move => {
                match c.unwrap() {
                    Key::Esc => {
                        mode = Mode::Normal;
                    }
                    _ => {}
                }
            }
        }
        
        let temp_pos = match stdout.cursor_pos() {
            Ok(pos) => pos,
            Err(e) => (1, 1)
        }; 

        write!(stdout, "{}", termion::cursor::Goto(1, 1));
        // draw the header
        write!(stdout, "Done\tIn Progress\tTodo");
        let mut todo_y = 2;
        let mut in_progress_y = 2;
        let mut done_y = 2;

        // draw the board
        for task in board.iter() {
            match task.status {
                Status::TODO => {
                    // move the cursor
                    // write out task title
                    write!(stdout,"{}{}", termion::cursor::Goto(1, todo_y), task.title);

                    // increment todo_y
                    todo_y += 1;
                }
                Status::IN_PROGRESS => {
                    // move the cursor
                    // write out task title
                    write!(stdout,"{}{}", termion::cursor::Goto(20, in_progress_y), task.title);

                    // increment todo_y
                    in_progress_y += 1;
                }
                Status::DONE => {
                    // move the cursor
                    // write out task title
                    write!(stdout,"{}{}", termion::cursor::Goto(40, done_y), task.title);

                    // increment todo_y
                    done_y += 1;
                }

            }
        }

        // reset cursor position
        write!(stdout, "{}", termion::cursor::Goto(temp_pos.0, temp_pos.1));

        // Flush again.
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
