extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{self, Fields};

#[proc_macro_derive(Form, attributes(input))]
pub fn formy_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_formy_derive(&ast)
}

// https://doc.rust-lang.org/book/ch19-06-macros.html

fn impl_formy_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let input_types = ["text", "password", "email",
                    "checkbox", "color", "date", "datetime-local",
                    "file", "hidden", "image", "month", "number",
                    "radio", "range", "reset", "search", "submit",
                    "tel", "time", "url", "week"];

    if let syn::Data::Struct(syn::DataStruct {
        struct_token,
        fields,
        semi_token,
    }) = &ast.data
    {
        let mut inputs = Vec::new();

        match fields {
            Fields::Named(named) => {
                for field in named.named.iter() {
                    let mut input_type = String::from("text");
                    // Parse macro attributes.
                    for attr in field.attrs.iter() {
                        if attr.path.is_ident("input") {
                            let meta = attr.parse_meta().unwrap();

                            if let syn::Meta::NameValue(v) = meta {
                                match v.lit {
                                    syn::Lit::Str(value) => {
                                        let val = value.value();

                                        if input_types.iter().any(|x| x.eq(&val)) {
                                            input_type = value.value();
                                        } else {
                                            let error_msg = format!("'input' macro attribute value must be a valid html input type ({:?}).", input_types);
                                            return quote_spanned! { 
                                                value.span()=> std::compile_error!(#error_msg);
                                            }.into();
                                        }
                                    }
                                    lit => {
                                        return quote_spanned! { lit.span()=> std::compile_error!("'input' macro attribute value must be a str.");}.into();
                                    }
                                }
                            } else {
                                return quote_spanned! { attr.path.span()=> std::compile_error!("'input' macro attribute must have a value.");}.into();
                            }
                        }
                    }
                    let name = field.ident.clone().unwrap();
                    let inp = format!("\t<input type='{}' name='{}'>\n", input_type, name);
                    inputs.push(inp);
                }
            }
            _ => {
                return quote_spanned! { struct_token.span=> std::compile_error!("This macro only accepts named fields."); }.into();
            }
        }

        let gen = quote! {
            impl Form for #name {
                fn to_html() -> String {
                    let mut html = String::new();
                    html.push_str("<form>\n");
                    #( html.push_str(#inputs); )*
                    html.push_str("</form>");

                    html
                }
            }
        };
        gen.into()
    } else {
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
