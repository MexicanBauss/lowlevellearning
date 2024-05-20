use core::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Element {
    pub message: String,
    pub status: bool,
}

impl Element {
    pub fn new(message: String) -> Element {
        Element { message, status: false }
    }

    pub fn recover(message: String) -> Element {
        let parts: Vec<&str> = message.split(":\t").collect();
        if parts.len() != 2 {
            panic!("Invalid format for recover method");
        }

        let status = match parts[1] {
            "true" => true,
            "false" => false,
            _ => panic!("Invalid status value in recover method"),
        };

        Element {
            message: parts[0].to_string(),
            status,
        }
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // write!(f, "[\"Message\": '{}',\"Status\": {}]", self.message, self.status)
        return write!(f, "{}:\t{}", self.message, self.status);
    }
}

#[derive(Debug)]
pub struct TodoList {
    pub elements: Vec<Element>,
}

impl TodoList {
    pub fn new(mut args: impl Iterator<Item = String>) -> TodoList {
        let mut elements = Vec::new();
        args.next(); // Skip the first argument

        // Process command-line arguments
        for arg in args {
            elements.push(Element::new(arg));
        }

        // Check if the .todo.txt file exists and read its contents
        if Path::new(".todo.txt").exists() {
            if let Ok(content) = fs::read_to_string(".todo.txt") {
                for line in content.lines() {
                    elements.push(Element::recover(line.to_string()));
                }
            }
        }

        TodoList { elements }
    }

    pub fn append(&mut self, element: Element) {
        self.elements.push(element);
    }

    pub fn reformat(&self) -> String {
        self.elements.iter()
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn remove_by_index(&mut self, index: usize) {
        if index < self.elements.len() {
            self.elements.remove(index);
        }
    }

    pub fn remove_by_message(&mut self, message: &str) {
        self.elements.retain(|e| e.message != message);
    }

    pub fn save(&self) {
        let path = Path::new(".todo.txt");
        let content = self.reformat();

        if path.exists() {
            // If the file exists, overwrite with the new content
            fs::write(path, content).unwrap();
        } else {
            // If the file does not exist, create it with the new content
            fs::write(path, content).unwrap();
        }
    }
}
