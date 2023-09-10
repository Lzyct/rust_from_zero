pub(crate) fn references_borrowing() {
    // borrowing concept
    let string_1 = String::from("Lzyct");
    // string_1 is borrowed by calculate_length function
    let len_string_1 = calculate_length_borrowing(string_1);

    // string_1 is not valid anymore
    println!("The length {}", len_string_1);
    // println!("string_1 {}",  string_1); // should be error because string_1 is not valid anymore

    let string_2 = String::from("Lzyct");
    let len_string2 = calculate_length_references(&string_2); // reference string_2 to calculate_length_references function
    println!("The length of {}, is {}", string_2, len_string2); // string_2 is still valid because it is not borrowed

    let mut string_3 = String::from("Hello");
    inject_lzyct(&mut string_3); // reference string_3 to inject_lzyct function
    // string_3 already added with ", Lzyct"
    println!("string_3: {}", string_3);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{}, {}", r1, r2); // we should use r1 and r2 here before r3 reference as mutable
    let r3 = &mut s; // BIG PROBLEM
    //println!("{}, {}", r1, r2); //error: we should use r1 and r2 here before r3 reference as mutable
    println!("{}", r3);
}

fn inject_lzyct(s: &mut String) {
    s.push_str(", Lzyct");
}

fn calculate_length_borrowing(s: String) -> usize {
    s.len()
}

fn calculate_length_references(s: &String) -> usize {
    s.len()
}