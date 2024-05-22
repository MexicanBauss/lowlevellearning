use std::fmt::Display;
use chrono::{DateTime, Utc};

pub struct User(String);

impl User {
    pub fn new(name: String) -> Self {
        User(name)
    }
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Message<'a> {
    author: &'a User,
    content: String,
    date: DateTime<Utc>,
}

impl<'a> Message<'a> {
    pub fn new(author: &'a User, content: String, date: DateTime<Utc>) -> Self {
        Message { author, content, date }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<'a> Display for Message<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} wrote on {}: {}", self.author, self.date, self.content)
    }
}

pub struct Chat<'a> {
    messages_sent: Vec<Message<'a>>,
    members: Vec<&'a User>,
}

impl<'a> Chat<'a> {
    pub fn new() -> Chat<'a> {
        Chat { 
            messages_sent: Vec::new(),
            members: Vec::new(),
        }
    }

    pub fn add_member(&mut self, user: &'a User) {
        self.members.push(user);
    }

    pub fn send_message(&mut self, message: Message<'a>) {
        if self.members.contains(&message.author) {
            self.messages_sent.push(message);
        } else {
            panic!("Error: User is not a member of the chat.");
        }
    }

    pub fn messages_sent(&self) -> &Vec<Message<'a>> {
        &self.messages_sent
    }

}