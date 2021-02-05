
pub use formy_derive::Form;

/// A trait for structs which can be parsed into a html form.
pub trait Form {
    fn to_html() -> String;
}

#[cfg(test)]
mod tests {
    #![allow(unused_variables)]
    #![allow(dead_code)]
    use crate::*;

    #[derive(Form)]
    struct UserLogin {
        username: String,
        #[input = "email"]
        email: String,
        #[input = "password"]
        password: String,
    }

    #[test]
    fn it_works() {
        let f = UserLogin::to_html();
        println!("{}", f);
    }
}
