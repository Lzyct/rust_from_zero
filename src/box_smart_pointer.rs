#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn box_smart_pointer() {
    let list = List::Cons(1,
                          Box::new(List::Cons(2,
                                              Box::new(List::Cons(3,
                                                                  Box::new(List::Nil))))));

    println!("list: {:?}", list);
}