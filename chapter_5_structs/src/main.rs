fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Teste"),
        email: String::from("Teste@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("Teste2@gmail.com");

    let user2 = User {
        email: String::from("teste3@gmail.com"),
        ..user1
    };

    let black = Color(0, 0, 0);

    let subject = AlwaysEqual;
}

struct AlwaysEqual;

struct Color(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
