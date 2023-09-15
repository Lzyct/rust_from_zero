use std::collections::HashMap;

pub fn collections() {
    let _a = [1, 2, 3];
    println!("a: {:?}", _a);

    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("v1: {:?}", v1);

    let mut v2 = vec![1, 2, 3];
    v2.push(4);
    v2.push(5);
    println!("v2: {:?}", v2);

    // access v2 data
    let forth = &v2[3];
    println!("forth: {}", forth);

    // access v2 data and handle if index out of range
    match v2.get(30) {
        None => {
            println!("There no element at index 30")
        }
        Some(v2) => {
            println!("v2: {}", v2)
        }
    }

    // iterate over v2
    for i in &v2 {
        println!("i: {}", i);
    }

    // modify v2
    for i2 in &mut v2 {
        *i2 += 50;
        println!("i2: {}", i2);
    }

    // use enum to store multiple types
    enum DataTypes {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // create a vector to store enum
    let v_data_types = vec![
        DataTypes::Int(10),
        DataTypes::Float(10.0),
        DataTypes::Text(String::from("Hello, Lzyct!")),
    ];

    // use match to get value from enum
    match v_data_types.get(3) {
        Some(DataTypes::Int(i)) => println!("i: {}", i),
        _ => println!("The value of index 3 is not integer"),
    }

    let mut string = String::from("Hello,");
    string.push_str(" Lzyct");
    string.push('!');

    println!("string: {}", string);

    // hashmap
    let blue = String::from("Blue");
    let red = String::from("Red");

    let mut scores = HashMap::new();
    // scores.insert(&blue, 10);
    scores.insert(&red, 20);

    // get value from hashmap and use error handler if key not exist
    match scores.get(&blue) {
        Some(score) => println!("score: {}", score),
        None => println!("There is no score for Blue"),
    }

    // not update for existing key in hashmap
    let yellow = String::from("Yellow");
    scores.entry(&yellow).or_insert(10);
    scores.entry(&yellow).or_insert(100);
    println!("scores: {:?}", scores); // score should be 10


    let string = "hello world from world hello";

    let mut count_word = HashMap::new();
    // ["hello", "world", "from", "world", "hello"]
    for word in string.split_whitespace() {
        println!("--------------------------------------------");
        let count = count_word.entry(word).or_insert(0);
        println!("1. {} : {}", word, count);
        *count += 1;
        println!("2. {} : {}", word, count);
        println!("3. count_word: {:?}", count_word[word]);
    }
    println!("count_word: {:?}", count_word)
}