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
}