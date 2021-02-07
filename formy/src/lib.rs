
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
        #[input(pattern = r"[\w]+")]
        #[label = "Username:"]
        username: String,
        #[input(type = "email", name = "real_email", class="red", id = "email")]
        email: String,
        #[input(type = "password")]
        #[label = "Password:"]
        password: String,
        some_field: String,
    }

    #[test]
    fn it_works() {
        let f = UserLogin::to_html();
        println!("{}", f);
    }
}
