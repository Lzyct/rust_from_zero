pub(crate) fn control_flow() {
    let number = 12;

    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    if number % 4 == 0 {
        println!("{} is divisible by 4", number);
    }
    if number % 3 == 0 {
        println!("{} is divisible by 3", number);
    }
    if number % 2 == 0 {
        println!("{} is divisible by 2", number);
    } else {
        println!("{} is not divisible by 4, 3, or 2", number);
    }

    let divisible_2 = if number % 5 == 0 {
        number
    } else {
        0
    };

    println!("divisible_2: {}", divisible_2);

    // loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result: {}", result);


    'test_loop: loop {
        let mut counter_1 = 0;
        println!("test loop");
        loop {
            counter_1 += 1;
            println!("nested loop");
            if counter_1 == 10 {
                break 'test_loop;
            }
        }
    }
    println!("test loop end");

    // while
    let mut while_number = 10;
    while while_number != 0 {
        println!("{}!", while_number);
        while_number -= 1;
    }
    println!("LIFTOFF!!!");

    let months = ["January", "February", "March", "April",
        "May", "June", "July", "August", "September", "October",
        "November", "Desember"];

    let mut index_month = 0;
    while index_month < months.len() {
        println!("The value index {} of month is: {}", index_month, months[index_month]);
        index_month += 1;
    }

    for month in months {
        println!("The value of month is: {}", month);
    }

    for number in (10..11).rev() {
        println!("{}!", number);
    }
}