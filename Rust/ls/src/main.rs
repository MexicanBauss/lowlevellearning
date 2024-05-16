use std::fs;
use std::env;

fn main() {
    let path = match env::args().nth(1) {
        Some(arg) => arg,
        _ => "./".to_string(),
    };

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                print!("{} ", entry.file_name().into_string().unwrap());
            }
        }
    } else {
        eprintln!("Error reading directory");
    };
    print!("\n");
}
