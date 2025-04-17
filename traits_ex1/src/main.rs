use aggregator::{notify, some_function, NewsArticle, Summary, Tweet};

fn main() {
    let tweet0 = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("New tweet: {}", tweet0.summarize());

    let article0 = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };
    println!("New article available! {}", article0.summarize());
    notify(&tweet0);
    notify(&article0);
    let res = some_function(&tweet0, &article0);
    println!("res: {}", res);
}
