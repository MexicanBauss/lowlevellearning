use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: i32,
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is {} years old.", self.name, self.age)
    }
}


fn main() {
    let name = String::from("John");
    let age = 24;

    let john = Person {
        name: &name,
        age: age,
    };

    println!("Display: {}", john);
    println!("Debug: {:?}", john);

}
