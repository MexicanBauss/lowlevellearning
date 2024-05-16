use std::fs;
use std::env;
use colored::Colorize;

fn main() {
    let path = match env::args().nth(1) {
        Some(arg) => arg,
        _ => "./".to_string(),
    };

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                if let Some(name) = file_name.to_str() {
                    if entry.file_type().map_or(false, |ft| ft.is_file()) {
                        print!("{} ", name.blue());
                    } else {
                        print!("{} ", name);
                    }
                }
            }
        }
    } else {
        eprintln!("Error reading directory");
    };
    print!("\n");
}
