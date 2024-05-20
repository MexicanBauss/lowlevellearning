use to_do_list::{Element, TodoList};
use console::Term;
use console::style;
use console::Key;

fn main() {
    let args = std::env::args();
    let mut todo_list = TodoList::new(args.skip(1)); // Skip the program name

    let term = Term::stdout();

    loop {
        term.clear_screen().unwrap();
        println!("{}", style("Todo list:").bold().underlined());
        println!("{}", todo_list.reformat());
        println!("\nOptions:");
        println!("1. Add a task");
        println!("2. Remove a task by index");
        println!("3. Save and exit");

        match term.read_key().unwrap() {
            Key::Char('1') => {
                println!("Enter the task:");
                if let Ok(task) = term.read_line() {
                    todo_list.append(Element::new(task));
                }
            },
            Key::Char('2') => {
                println!("Enter the index of the task to remove:");
                if let Ok(index_str) = term.read_line() {
                    if let Ok(index) = index_str.trim().parse::<usize>() {
                        todo_list.remove_by_index(index);
                    } else {
                        println!("Invalid index");
                    }
                }
            },
            Key::Char('3') => {
                todo_list.save();
                break;
            },
            _ => (),
        }
    }
}
