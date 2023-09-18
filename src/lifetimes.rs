pub fn lifetimes() {
    let s1 = String::from("abcd");
    let s2 = String::from("xyz");
    let result = longest_string(s1.as_str(), s2.as_str());
    println!("The longest string is {}", result);
}

// Error missing lifetime specifier
// fn longest_string(x: &str, y: & str) -> & str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }


// specifier lifetime with 'a generic lifetime
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}