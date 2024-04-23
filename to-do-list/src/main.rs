use std::env::Args;

#[derive(Debug)]
struct TodoElement {
    message: String,
    status: bool,
}
#[derive(Debug)]
struct TodoList {
    content: Vec<TodoElement>,
}
impl TodoList {
    fn new() -> TodoList {
        TodoList { content: Vec::new() }
    }

    fn add_elements(&mut self, message: String) {
        let new_element = TodoElement {
            message: message,
            status: false,
        };
        self.content.push(new_element);
    }
}
fn main() {

    

    let mut todolist = TodoList::new();

    todolist.add_elements(String::from("Mejorar el código"));
    todolist.add_elements(String::from("Hacer palomitas"));
    get_elements(&todolist);

}
fn get_elements(list: &TodoList) {
    println!("{:?}", list);
}