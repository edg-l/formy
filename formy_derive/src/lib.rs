extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, Fields};

#[proc_macro_derive(Form)]
pub fn formy_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_formy_derive(&ast)
}

// https://doc.rust-lang.org/book/ch19-06-macros.html

fn impl_formy_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    if let syn::Data::Struct(syn::DataStruct {
        struct_token,
        fields,
        semi_token,
    }) = &ast.data
    {
        let mut field_vec_innards = quote!();

        match fields {
            Fields::Named(named) => {
                for field in named.named.iter() {
                    let name = field.ident.clone().unwrap();
                    field_vec_innards.extend(quote!(stringify!(#name),));
                }
            }
            _ => unimplemented!(),
        }
        let fields_vec = quote!(vec![#field_vec_innards]);

        let gen = quote! {
            impl Form for #name {
                fn to_html() -> String {
                    let mut html = String::new();
                    html.push_str("<form>\n");
                    for f in #fields_vec.iter() {
                        html.push_str(&format!("<input type='text' id='{}' name='{}'>\n", f, f));
                    }
                    html.push_str("</form>");

                    html
                }
            }
        };
        gen.into()
    } else {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
