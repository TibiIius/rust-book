use std::fmt::Display;

fn main() {
  let article = NewsArticle {
    headline: "Anime Neko Girls".to_string(),
    body: "In this article, I'm going to talk about neko girls in anime UwU.".to_string(),
    author: "Tim B.".to_string(),
  };

  println!("{}", article.summarize());
}

trait Summary {
  fn summarize(&self) -> String;
}

struct NewsArticle {
  headline: String,
  body: String,
  author: String,
}

impl Summary for NewsArticle {
  fn summarize(&self) -> String {
    format!(
      "Article \"{}\", written by \"{}\": \"{:?}\".",
      self.headline,
      self.author,
      self.body.get(0..300).unwrap_or(&self.body[..])
    )
  }
}

// do this
fn do_stuff(e: &impl Summary) -> String {
  e.summarize()
}

// or this (functionally equivalent)
fn do_stuff_but_better<T: Summary>(e: T) -> String {
  e.summarize()
}

// or this (PogT right here)
fn do_stuff_the_best<T, U>(e_1: T, e_2: U) -> i32
where
  T: Display + Summary,
  U: Summary,
{
  1337
}
