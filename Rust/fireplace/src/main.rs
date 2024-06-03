#![allow(dead_code)]
use termion::terminal_size;
use rand::Rng;
use std::thread;
use std::time::Duration;

struct CountBuffer {
    table: Vec<Vec<u8>>,
    render_height: u16,
    render_width: u16,
}

impl CountBuffer {
    fn new(width: u16, height: u16) -> Self {
        // Initialize CountBuffer with some default values
        let table = vec![vec![0; width as usize]; height as usize];
        CountBuffer { table, render_height: height, render_width: width }
    }
    fn update_values(&mut self) {
        // Generation and vertical movement
        for i in 0..self.table.len() {
            for j in 0..self.table[i].len() {
                if i as u16 == self.render_height - 1 {
                    let random_number: u8 = rand::thread_rng().gen_range(7..=20);
                    self.table[i][j] = random_number;
                } else {
                    let remove_num: u8 = rand::thread_rng().gen_range(0..3);
                    if self.table[i + 1][j] > 0 && self.table[i + 1][j] >= remove_num {
                        self.table[i][j] = self.table[i + 1][j] - remove_num;
                    } else {
                        self.table[i][j] = 0;
                    }
                }
            }
        }
    }
    fn render_frame(&self) {
        print!("\x1b[H");

        for columns in self.table.iter() {
            let mut row_content: Vec<char> = vec![];
            for value in columns.iter() {
                let val = match value {
                    0 => ' ',
                    1 => '.',
                    2 => ':',
                    3 => '|',
                    4 => '=',
                    5 => '%',
                    6 => '#',
                    _ => '@',
                };
                row_content.push(val);
            }
            let row_string: String = row_content.iter().collect();
            println!("{}", row_string);
        }
    }
}

/// Main function that initializes a CountBuffer with terminal size, updates its values and renders frames in a loop with a delay of 80 milliseconds.
/// 
/// # Examples
/// 
/// ```
/// fn main() {
///     let (width, height) = terminal_size().unwrap();
///     let mut count_buffer = CountBuffer::new(width, height);
/// 
///     loop {
///         count_buffer.update_values();
///         count_buffer.render_frame();
///         thread::sleep(Duration::from_millis(80));
///     }
/// }
///
fn main() {
    
    let (width, height) = match terminal_size() {
        Ok((w, h)) => (w, h),
        Err(e) => {
            eprintln!("Error getting terminal size: {}", e);
            std::process::exit(1);
        }
    };

    let mut count_buffer = CountBuffer::new(width, height);

    loop {
        count_buffer.update_values();
        count_buffer.render_frame();
        thread::sleep(Duration::from_millis(80));
    }

}
