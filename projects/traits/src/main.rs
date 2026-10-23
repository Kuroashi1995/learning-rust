use aggregator::{ self, Article, SocialPost, Summary};

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//also we can specify traits as return types
fn returns_summarizable(username: String) -> impl Summary {
    SocialPost {
        username,
        content: String::from("some content"),
        reply: false,
        repost: false
    }
}

fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        repost: false,
    };

    let article = Article {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pitsburg, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("Some text, really, believe me"),
    };

    println!("1 new post: {}", post.summarize());
    println!("New article available! {}", article.summarize());

    notify(&article);

    let summarizable = returns_summarizable(String::from("Tony Stark"));

    println!("A new summarizable has appeared: {}", summarizable.summarize())

}
