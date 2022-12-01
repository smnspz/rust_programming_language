fn main() {
    
    let mut user1 = build_user(String::from("example@example.com"), String::from("username"));

    let mut user2 = User {
        username: String::from("example@example.com"),
        email: String::from("username"),
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    user1.active = false;

    let rect1 = Rectangle {
        width: 30,
        heigth: 50,
    };

    let rect2 = Rectangle { width: 10, heigth: 40 };
    let rect3 = Rectangle { width: 60, heigth: 45 };

    let square = Rectangle::square(32);

    // Use :#? for better struct formatting
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!("This is a square: {:#?}", square);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.heigth
    }
 
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() < rect.area()
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, heigth: size }
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

