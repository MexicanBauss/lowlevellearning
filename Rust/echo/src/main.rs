use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut i: u8 = 0;
    for arg in args {
        if i > 0 {
            print!("{} ", arg);
        }
        i += 1;
    }
    print!("\n");
}
