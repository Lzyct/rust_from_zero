use std::env;

struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a filename string"),
        };

        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let case_sensitive = std::env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn search<'search>(query: &str, contents: &'search str) -> Vec<&'search str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line)
    //     }
    // }
    // results

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn iterators2() {
   let src= search("TODO", "TODO");
    println!("src: {:?}", src);
}