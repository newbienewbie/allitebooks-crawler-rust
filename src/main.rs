
extern crate select;
extern crate regex;
extern crate reqwest;

mod crawler;


pub fn main() {

    let c=crawler::scheme::crawler::Crawler::new("http://www.allitebooks.com/".to_string());
    c.crawl("http://www.allitebooks.com/");
}
