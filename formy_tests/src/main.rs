#![allow(dead_code)]

use formy::Form;

#[derive(Form)]
struct UserLogin {
    #[input(pattern = r"[\w]+")]
    #[label = "Username:"]
    username: String,
    #[input(type = "email", name = "real_email", class="py-4", id = "email")]
    email: String,
    #[input(type = "password")]
    #[label = "Password:"]
    password: String,
    some_field: String,
}


fn main() {
    let form = UserLogin::to_form();
    println!("{}", form);
}
