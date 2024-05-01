use std::io;

fn main() {
    let vocals = vec!['a', 'e', 'i', 'o', 'u']; 
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Unable to read line");

    word = word.trim().to_string();
    
    let first_letter = word.chars().next().unwrap(); 
    let mut new_word = String::new();

    if vocals.contains(&first_letter) {
        new_word.push_str(&word); 
        new_word.push_str("hay");
    } else {
        new_word.push_str(&word[1..]); 
        new_word.push(first_letter); 
        new_word.push_str("ay"); 
    }

    println!("{}", new_word);
}
