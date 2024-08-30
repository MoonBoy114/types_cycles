use std::{fmt::{Debug, Display}, iter::Sum};

// pub trait Summary {
//     fn summarize(&self) -> String;
// }
pub struct NewsArticle {
    pub headline: String,
    pub location: String, 
    pub author: String,
    pub content: String,
}
// impl Summary for NewsArticle {
//     fn summarize_author(&self) -> String {      
//     }
//     fn summarize(&self) -> String {
//         format!("(Читать дальше в {})", self.summarize())
//     }
// }
// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, {} ({})", self.headline, self.author, self.location)
//     }
// }
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Читать дальше в {}", self.summarize_author())
    }
}
impl Summary for Tweet {
  
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify(item: impl Summary) {
    println!("Срочные новости! {}", item.summarize());
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}



