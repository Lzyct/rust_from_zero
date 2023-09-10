#[derive(Debug)]
enum LoginProvider {
    Facebook,
    Google,
    Twitter,
    Email,
}


#[derive(Debug)]
struct User {
    username: String,
    email: String,
    login_with: LoginProvider,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn increase_sign_in_count(&mut self) {
        self.sign_in_count += 1;
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u64,
    height: u64,
}

impl Rectangle {
    fn area(&self) -> u64 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn can_hold_other(&self, other: &User) -> bool {
        self.width > other.sign_in_count && self.height > other.sign_in_count
    }
}

impl Rectangle {
    fn square(size: u64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub(crate) fn structs() {
    let mut user1 = User {
        email: String::from("hey.mudassir@gmail.com"),
        username: String::from("user1"),
        login_with: LoginProvider::Email,
        sign_in_count: 1,
        active: true,
    };

    // update user1 to active false
    user1.active = false;

    // create user 2 from user_builder
    let user2 = user_builder(
        String::from("lzyct@lazycatlabs.com"),
        String::from("user2"),
        LoginProvider::Google,
    );
    // user2.increase_sign_in_count(); // should be error because user2 is not mutable

    println!("user2 login with {:#?}", user2);
    let mut user3 = User {
        email: String::from("lazycatlabs@gmail.com"),
        login_with: LoginProvider::Facebook,
        username: String::from("user3"),
        ..user2 // copy all field from user2
    };

    println!("user1 login with {:#?}", user1);
    println!("user3 login with {:#?}", user3);

    println!("user3 increase sign from impl");
    user3.increase_sign_in_count();
    println!("user3 login with {:#?}", user3);

    // Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    let rect2 = Rectangle::square(20);
    let rect3 = Rectangle {
        width: 50,
        height: 60,
    };

    println!("rect1 is {:#?}", rect1);
    println!("rect1 area is {}", rect1.area());
    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3? {}", rect1.can_hold(&rect3));
    println!("rect1 can hold user1? {}", rect1.can_hold_other(&user1));
    println!("rect1 can hold user3? {}", rect1.can_hold_other(&user3));
}

fn user_builder(email: String, username: String, login_with: LoginProvider) -> User {
    User {
        email, // if use same name with struct field, we can use this shortcut
        username,
        login_with,
        sign_in_count: 1,
        active: true,
    }
}