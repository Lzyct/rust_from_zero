fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_three<T>(f: T, x: i32) -> i32
    where T: Fn(i32) -> i32 {
    f(x) + f(x) + f(x)
}

pub fn advance_function_closures() {
    let add_two_fn = add_three(add_one, 2); // 2+1 + 2+1 + 2+1 = 9
    println!("add_two_fn: {}", add_two_fn);

    // parse list int to list string
    let list_of_int = vec![1, 2, 3];
    let list_of_string: Vec<String> = list_of_int
        .iter()
        .map(|i| i.to_string())
        .collect();

    println!("list_of_string: {:?}", list_of_string);

    let test = return_closure(6);
    println!("test: {:?}", test(5));
}

fn return_closure(b: i32) -> Box<dyn Fn(i32) -> i32> {
    if b < 5 {
        Box::new(move |x| x + b)
    } else {
        Box::new(move |x| x - b)
    }
}