pub(crate) fn variable_mutability() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // const
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    // shadowing
    let x = 10;
    println!("The value of x shadowing is: {}", x);

    // shadowing with different type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces shadowing is: {}", spaces);

}