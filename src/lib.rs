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

pub fn notify2<T: Summary>(item: T, item2: T) {

}

pub fn notify3<T: Summary + Display>(item: T) {

}

fn some_function<T, U>(t: T, u: U) -> i32 where T: Display + Clone,
    U: Clone + Debug {
        
    }