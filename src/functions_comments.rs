// 3. Common Programming Concepts : Functions and Comments
pub(crate) fn functions_comments() {
    println!("Five {}", five());
    println!("Sum 5+5 {}", sum(5, 5));

    // Call weather_temp function
    weather_temp(30, 'C');
}

// Return five
fn five() -> i32 {
    5
}

// Sum two numbers
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

// Print weather temperature
fn weather_temp(temp: i32, unit_label: char) {
    println!("The temperature is {} {}", temp, unit_label);
}