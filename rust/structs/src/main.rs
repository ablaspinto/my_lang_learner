struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
fn build_user(name: String, email: String) -> User {
    User {
        email,
        user_name: name,
        active: true,
        sign_in_count: 1,
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn main() {
    let rec = Rectangle {
        width: 40,
        height: 20,
    };

    println!("rectangle area is: {}", area(&rec));

    let user1 = User {
        email: String::from("ablaspinto@gmail.com"),
        user_name: String::from("alfonso_abp"),
        active: true,
        sign_in_count: 1,
    };
    let name: String = user1.user_name;
    let user2 = build_user(String::from("bob"), String::from("bob@gmail.com"));
}
