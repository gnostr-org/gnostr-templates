pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Characters {
    fn characters(&self) -> usize;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl Characters for Tweet {
    fn characters(&self) -> usize {
        self.content.len()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub trait IsThisExtensionMethods {
    fn dunno(&self) -> String;
}

// ZOMG extension functions!
impl IsThisExtensionMethods for String {
    fn dunno(&self) -> String {
        self.to_ascii_uppercase()
    }
}

fn combined_types(tweet: impl Summary + Characters) {
    println!(
        "Tweet of {} chars: {}",
        tweet.characters(),
        tweet.summarize()
    );
}

pub(crate) fn traits() {
    let news = NewsArticle {
        headline: "".to_string(),
        location: "".to_string(),
        author: "".to_string(),
        content: "".to_string(),
    };
    println!("{}", news.content);

    let tweet = Tweet {
        username: "Richo".to_string(),
        content: "Hello World".to_string(),
        reply: false,
        retweet: false,
    };

    let chars: &dyn Characters = &tweet as &dyn Characters;
    println!("{}", chars.characters());
    let summ: &dyn Summary = &tweet as &dyn Summary;
    println!("{}", summ.summarize());

    combined_types(tweet);
    println!("extension method?: {}", "asd".to_string().dunno())
}
