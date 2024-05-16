#[derive(Debug)]
pub struct Tweet {
    handle: String,
    content: String,
}

pub struct Journal {
    author: String,
    title: String,
}

pub trait Printable {
    fn reformat(&self) -> String;
}

impl Printable for Tweet {
    fn reformat(&self) -> String {
        format!("User @{} said: {}", self.handle, self.content)
    }
}

impl Printable for Journal {
    fn reformat(&self) -> String {
        format!("World renowned {} has published his latest article: {}", self.author, self.title)
    }
}

impl Tweet {
    fn cmp_handle(&self, other: &Tweet) -> bool {
        match self.handle == other.handle {
            true => true,
            _ => false,
        }
    }
}

fn main() {
    
    // Tweets
    let tweet1 = Tweet {
        handle: "Prime".to_string(),
        content: "Making an amazing vid about $$$".to_string(),
    };
    let tweet2 = Tweet {
        handle: "Prime".to_string(),
        content: "Rust made by ze gae?".to_string(),
    };
    match tweet1.cmp_handle(&tweet2) {
        true => println!("It's the same user!"),
        _ => println!("Some other blud"),
    };

    // Journalist bros
    let paper1 = Journal {
        author: "Jeremy Clarkson".to_string(),
        title: "The new Dacia Sandero is still garbage".to_string(),
    };

    println!("NEWS FLASH!!! {}", paper1.reformat());

}
