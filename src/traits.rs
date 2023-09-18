pub struct Articles {
    pub author: String,
    pub title: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Articles {
    fn summarize_auth(&self) -> String {
        String::from(&self.author)
    }
}

impl Summary for Tweet {
    fn summarize_auth(&self) -> String {
        String::from(&self.username)
    }

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub trait Summary {
    fn summarize_auth(&self) -> String;
    fn summarize(&self) -> String {
        format!("(What next?) {}", self.summarize_auth())
    }
}

pub fn notify(item: &impl Summary) {
    println!("WTF?! {}", item.summarize());
}


pub fn _some_functions2<T: Summary + Clone, U: Clone + Summary>(t: T, u: U) -> i32 {
    3
}

// same as above but more readable
pub fn _some_functions<T, U>(t: T, u: U) -> i32
    where T: Summary + Clone,
          U: Clone + Summary
{
    3
}

fn return_summarizable() -> impl Summary {
    Articles {
        author: String::from("Lazycat Labs"),
        title: String::from("Rust from zero"),
        content: String::from("Rust is a good language"),
    }
}

pub fn traits() {
    let article = Articles {
        author: String::from("Lzyct"),
        title: String::from("Rust from zero"),
        content: String::from("Rust is a good language"),
    };
    let tweet = Tweet {
        username: String::from("Lzyct"),
        content: String::from("Tweet from Lzyct"),
        reply: false,
        retweet: false,
    };


    println!("tweet: {}", tweet.summarize());
    println!("article: {}", article.summarize());
    notify(&article);

    println!("return_summarizable: {}", return_summarizable().summarize());
}