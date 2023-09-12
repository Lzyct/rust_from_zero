enum LoginResult {
    Success(Profile),
    Failure(FailureState),
    Loading(bool),
}

#[derive(Debug)]
enum FailureState {
    InvalidPassword,
    InvalidUsername,
    InvalidEmail,
}

struct Profile {
    username: String,
    email: String,
    age: u8,
}

impl LoginResult {
    fn message() -> String {
        String::from("Hello, Lzyct!")
    }
}

enum Dynamic<T> {
    Value(T),
    None,
}

pub(crate) fn enums_patter_matching() {
    let mut login_result1 = LoginResult::Loading(true);
    login_result1 = LoginResult::Success(Profile {
        username: String::from("Lzyct"),
        email: String::from("hey.mudassir@gmail.com"),
        age: 25,
    });
    login_result1 = LoginResult::Failure(FailureState::InvalidEmail);
    match login_result1 {
        LoginResult::Success(profile) => {
            println!("Welcome, {}", profile.email);
        }
        LoginResult::Failure(failureState) => {
            println!("failure {:?}", failureState);
        }
        LoginResult::Loading(_) => {
            println!("{}", LoginResult::message());
        }
    }

    let num6: i8 = 6;
    let value6: Option<i8> = None;
    let sum = num6 + value6.unwrap_or(9);
    println!("sum {}", sum); // 45

    current_state(LoginResult::Success(Profile {
        username: String::from("Lzyct"),
        email: String::from("hey.mudassir@gmail.com"),
        age: 25,
    }));

    let eleven = Some(11);
    let eleven_plus_two = plus_two(eleven);
    println!("{}", eleven_plus_two.unwrap_or(0));

    let three = Some(3);

    match three {
        Some(3) => {
            println!("three")
        }
        _ => ()
    }

    if let Some(3) = three {
        println!("hi,three")
    }
}

fn plus_two(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 2),
        _ => None, // _ is a catchall value
    }
}

fn current_state(result: LoginResult) -> u8 {
    match result {
        LoginResult::Success(profile) => {
            println!("Welcome, {}", profile.email);
            1
        }
        LoginResult::Failure(failureState) => {
            println!("failure {:?}", failureState);
            2
        }
        LoginResult::Loading(_) => {
            println!("{}", LoginResult::message());
            3
        }
    }
}