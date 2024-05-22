use pattern_n_matching::*;
use chrono::Utc;

fn main() {
    let user1 = User::new("Alice".to_string());
    let user2 = User::new("Bob".to_string());

    let mut chat = Chat::new();
    chat.add_member(&user1);
    chat.add_member(&user2);

    let message1 = Message::new(&user1, "Hello, everyone!".to_string(), Utc::now());
    let message2 = Message::new(&user2, "Hi, Alice!".to_string(), Utc::now());

    chat.send_message(message1);
    chat.send_message(message2);

    for message in chat.messages_sent() {
        println!("{}", message);
    }
}
