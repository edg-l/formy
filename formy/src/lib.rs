use formy_derive::Form;

pub trait Form {
    fn to_html() -> String;
}


#[derive(Debug, Form)]
struct UserLogin {
    username: String,
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
