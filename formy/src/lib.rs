#![allow(unused_variables)]
#![allow(dead_code)]

use formy_derive::Form;

pub trait Form {
    fn to_html() -> String;
}

#[derive(Form)]
struct UserLogin {
    username: String,
    #[input = "email"]
    email: String,
    #[input = "password"]
    password: String,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let f = UserLogin::to_html();
        println!("{}", f);
    }
}
