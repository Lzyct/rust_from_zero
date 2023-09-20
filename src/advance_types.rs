use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}>", self.0.join(","))
    }
}

pub fn advance_types() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);


    type Integers = i32;

    let x: i32 = 7;
    let y: Integers = 3;
    println!("x + y = {}", x + y);

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));
    let takes_long_type = |f: Thunk| {
        f();
    };
    let returns_long_type = || -> Thunk {
        Box::new(|| println!("hi"))
    };

    takes_long_type(f);
    returns_long_type()();
}