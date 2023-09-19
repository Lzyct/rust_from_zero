struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum MessageOne {
    Hello { id: i32 }
}

pub fn pattern_syntax() {
    let x = Some(10);
    let y = 12;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // match with or
    match y {
        1 | 12 => println!("Got 1 or 12"),
        _ => println!("Default case, y = {:?}", y),
    }

    // match with range
    let j = 'j';
    match y {
        1..=12 => println!("Got a number between 1 and 12, y = {:?}", y),
        _ => println!("Default case, y = {:?}", y)
    }

    match j {
        'a'..='k' => println!("{} is between a..k", j),
        'l'..='z' => println!("{} is between l..z", j),
        _ => println!("something else")
    }

    // destructuring
    let p = Point { x: 0, y: 7 };
    let Point { x, y } = p;
    println!("x = {}, y = {}", x, y);

    let msg = Message::ChangeColor(1, 2, 3);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        }
    }

    let numbers = (1, 2, 3, 4, 5);
    // use range to ignore some values
    match numbers {
        (first, .., before_last, last) => {
            println!("first: {},before_last: {}, last: {}", first, before_last, last);
        }
    }

    let num1 = Some(69);

    // match with condition
    match num1 {
        None => (),
        Some(x) if x < 70 => println!("less than 70"),
        Some(_) => println!("more than 70")
    }

    let x1 = 7;
    let y1 = false;
    match x1 {
        6 | 7 | 8 if y1 => println!("Yes!"),
        _ => println!("No!")
    }

    // save value from match
    let message_one = MessageOne::Hello { id: 5 };
    match message_one {
        // save value from match use @
        MessageOne::Hello { id: id_variable @ 3..=7 } =>
            println!("Found an id in range: {}", id_variable),
        MessageOne::Hello { id: 10..=12 } =>
            println!("Found an id in another range"),
        MessageOne::Hello { id } => println!("Found some other id: {}", id),
    }
}