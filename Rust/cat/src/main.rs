use std::env;
use std::fs;

fn main() {
    for (i, file) in env::args().enumerate() {
        if i > 0 {
            let cur_content = match fs::read_to_string(file){
                Ok(content) => content,
                _ => continue,
            };
            print!("{}", cur_content);
        }
    }
}
