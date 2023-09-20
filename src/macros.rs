macro_rules! hello_lzyct {
    ($name:expr) => {
        println!("Hello, {}!",$name);
    };
}

macro_rules! repeat_print {
    // match a single expression
    ($value:expr) => {
        println!("value = {}", $value);
    };
    ($($value:expr),*) => {
        $(
            println!("value = {}", $value);
        )*
    };
}

pub fn macros() {
    hello_lzyct!(123);
    repeat_print!(1,2,3,4,5);
    repeat_print!("X");
}