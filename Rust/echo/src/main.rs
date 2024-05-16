use std::env;

fn main() {
    for (i, arg) in env::args().enumerate() {
        if i > 0 {
            print!("{} ", arg);
        }
    }
    print!("\n");
}
