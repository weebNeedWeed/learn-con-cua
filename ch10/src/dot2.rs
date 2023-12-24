use std::fmt::{Display, Formatter};

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("read more")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl<T: Display> Summary for T {}

impl Summary for NewsArticle {}

pub fn notify<T>(item: &T)
where
    T: Summary,
{
    println!("New items: {}", item.summarize())
}
