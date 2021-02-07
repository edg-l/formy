//! A derive macro to generate HTML forms from structs.
//!
//! Currently in early development, only input with all its attributes is handled right now.
//!
//! ```rust
//! use formy::Form;
//!
//! #[derive(Form)]
//! struct UserLogin {
//!     #[input(pattern = r"[\w]+")]
//!     #[label = "Username:"]
//!     username: String,
//!     #[input(type = "email", name = "real_email", class="py-4", id = "email")]
//!     email: String,
//!     #[input(type = "password")]
//!     #[label = "Password:"]
//!     password: String,
//!     some_field: String,
//! }
//!
//! let form = UserLogin::to_html();
//! ```
//!
//! ## TODO:
//! 
//! - [ ] \<select>
//! - [ ] \<textarea>
//! - [ ] \<button>
//! - [ ] \<fieldset>
//! - [ ] \<legend>
//! - [ ] \<datalist>
//! - [ ] \<output>
//! - [ ] \<option>
//! - [ ] \<optgroup>

/// The derive macro.
pub use formy_derive::Form;

/// A trait for structs which can be parsed into a html form.
pub trait Form {
    fn to_form() -> String;
}
