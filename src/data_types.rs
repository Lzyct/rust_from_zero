use std::io;

// 3. Common Programming Concepts : Data Types
pub(crate) fn data_types() {
    // Should define data types
    let guess: u8 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let x: f32 = 2.0;
    let y = 2.0;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    // +
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // -
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // *
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // /
    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {}", quotient);

    // mod
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);

    // boolean
    let t = true;
    println!("The value of t is: {}", t);
    let f: bool = false;
    println!("The value of f is: {}", f);

    // char
    let c = '❤';
    println!("The value of c is: {}", c);

    let x = '𝕏';
    println!("The value of x is: {}", x);

    let cat = '😻';
    println!("The value of cat is: {}", cat);

    // tuple
    let mixed = (500, 6.1, "Lzyct");
    println!("The value of mixed is: {:?}", mixed);

    let (x, y, z) = mixed;
    println!("The value of x: {}, y: {}, z: {} ", x, y, z);

    // access tuple by index
    let five_hundred = mixed.0;
    let six_point_one = mixed.1;
    let lzyct = mixed.2;
    println!("The value of five_hundred: {}, six_point_one: {}, lzyct: {}", five_hundred, six_point_one, lzyct);

    // array
    // type i32 with length 5
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    // generate array with default value 3 and length 5;
    let b = [3; 5];
    println!("The value of b is: {:?}", b);


    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September",
        "October", "November", "December"];
    let random_month = months[5];
    println!("The value of jan is: {}", random_month);

    println!("Please input month in number!");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let month = months[index - 1];
    println!("The value of month is: {}", month);
}