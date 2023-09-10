pub(crate) fn slice() {
    let x = [1, 2, 3, 4, 5];
    let slice = &x[1..3]; // exclude 3
    println!("slice: {:?}", slice);

    let s = String::from("hello world");
    let hello = &s[..5]; // start from index 0 until 5
    let world = &s[6..]; // start from index 6 until end
    println!("hello: {}, world: {}", hello, world);
}