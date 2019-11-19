use std::fmt;

struct User {
    email: String,
    username: String,
    sign_in_count: u64,
    active: bool,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User: {}, {}, {}, {}",
            self.email, self.username, self.sign_in_count, self.active
        )
    }
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = build_user(String::from("foo"), String::from("foo@bar.com"));

    user1.username = String::from("bar");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{}", user2);
}
