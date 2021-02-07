extern crate proc_macro;

use inflector::Inflector;
use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{self, Fields};

#[proc_macro_derive(Form, attributes(input, label))]
pub fn formy_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_formy_derive(&ast)
}

// https://doc.rust-lang.org/book/ch19-06-macros.html

struct InputAttribute {
    pub name: &'static str,
    pub any_value: bool,
    pub accepted_values: &'static[&'static str],
}

fn get_all_possible_attributes() -> Vec<InputAttribute> {
    let mut x = vec![];

    // Add all: https://www.w3schools.com/tags/tag_input.asp

    x.push(InputAttribute {
        name: "type",
        any_value: false,
        accepted_values: &[
            "text",
            "password",
            "email",
            "checkbox",
            "color",
            "date",
            "datetime-local",
            "file",
            "hidden",
            "image",
            "month",
            "number",
            "radio",
            "range",
            "reset",
            "search",
            "submit",
            "tel",
            "time",
            "url",
            "week",
        ],
    });
    x.push(InputAttribute {
        name: "alt",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "name",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "id",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "class",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "autocomplete",
        any_value: false,
        accepted_values: &["on", "off"],
    });
    x.push(InputAttribute {
        name: "autofocus",
        any_value: false,
        accepted_values: &["autofocus"],
    });
    x.push(InputAttribute {
        name: "checked",
        any_value: false,
        accepted_values: &["checked"],
    });
    x.push(InputAttribute {
        name: "dirname",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "disabled",
        any_value: false,
        accepted_values: &["disabled"],
    });
    x.push(InputAttribute {
        name: "form",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "formaction",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "formenctype",
        any_value: false,
        accepted_values: &["application/x-www-form-urlencoded", "multipart/form-data", "text/plain"],
    });
    x.push(InputAttribute {
        name: "formmethod",
        any_value: false,
        accepted_values: &["post", "get"],
    });
    x.push(InputAttribute {
        name: "formnovalidate",
        any_value: false,
        accepted_values: &["formnovalidate"],
    });
    x.push(InputAttribute {
        name: "formtarget",
        any_value: false,
        accepted_values: &["_blank", "_self", "_parent", "_top"],
    });
    x.push(InputAttribute {
        name: "height",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "width",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "list",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "max",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "min",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "minlenght",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "multiple",
        any_value: false,
        accepted_values: &["multiple"],
    });
    x.push(InputAttribute {
        name: "pattern",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "placeholder",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "readonly",
        any_value: false,
        accepted_values: &["readonly"],
    });
    x.push(InputAttribute {
        name: "required",
        any_value: false,
        accepted_values: &["required"],
    });
    x.push(InputAttribute {
        name: "size",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "src",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "step",
        any_value: true,
        accepted_values: &[],
    });
    x.push(InputAttribute {
        name: "value",
        any_value: true,
        accepted_values: &[],
    });

    x
}

fn get_meta_list(nested_meta: &syn::MetaList) -> Result<Vec<(&syn::Path, &syn::Lit)>, TokenStream> {
    let mut list = vec![];
    for v in &nested_meta.nested {
        match v {
            syn::NestedMeta::Meta(m) => {
                if let syn::Meta::NameValue(value) = &m {
                    list.push((&value.path, &value.lit));
                } else {
                    return Err(
                        quote_spanned! {m.span()=> compile_error!("Must be a named value.")}.into(),
                    );
                }
            }
            x => {
                return Err(
                    quote_spanned! {x.span()=> compile_error!("Invalid meta type.")}.into(),
                );
            }
        }
    }

    Ok(list)
}

fn impl_formy_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let input_types = get_all_possible_attributes();

    if let syn::Data::Struct(syn::DataStruct {
        struct_token,
        fields,
        semi_token: _,
    }) = &ast.data
    {
        let mut inputs = Vec::new();

        match fields {
            Fields::Named(named) => {
                for field in named.named.iter() {
                    // Defaults
                    let mut input_attributes = Vec::new();

                    let mut name_added = false;
                    let mut id_added = false;
                    let mut input_name = field.ident.clone().unwrap().to_string();
                    let mut input_id = field.ident.clone().unwrap().to_string();
                    let mut label = None;

                    // Parse macro attributes.
                    for attr in field.attrs.iter() {
                        // Everything is under the meta list attribute "input",
                        // like #[input(name = "myfield")]
                        //
                        if attr.path.is_ident("input") {
                            let meta = attr.parse_meta().unwrap();

                            if let syn::Meta::List(nested_meta) = meta {
                                match get_meta_list(&nested_meta) {
                                    Ok(ref values) => {
                                        for value in values {
                                            match value.1 {
                                                syn::Lit::Str(valstr) => {
                                                    let val = valstr.value();

                                                    let mut found = false;

                                                    for inp_atr in &input_types {
                                                        if value.0.is_ident(inp_atr.name) {
                                                            found = true;

                                                            if value.0.is_ident("name") {
                                                                name_added = true;
                                                                input_name = val.clone();
                                                            }
                                                            else if value.0.is_ident("id") {
                                                                id_added = true;
                                                                input_id = val.clone();
                                                            }

                                                            if inp_atr.any_value {
                                                                input_attributes.push(format!("{}=\"{}\"", inp_atr.name, val));
                                                            }
                                                            else if inp_atr.accepted_values.iter().any(|x| x.eq(&val)) {
                                                                input_attributes.push(format!("{}=\"{}\"", inp_atr.name, val));
                                                            }
                                                            else {
                                                                let error_msg = 
                                                                    format!("'input' macro attribute value for '{}' must be one of the following: {:?}.", 
                                                                    inp_atr.name,
                                                                    inp_atr.accepted_values
                                                                    );
                                                                return quote_spanned!{valstr.span()=> compile_error!(#error_msg);}.into();
                                                            }
                                                            break;
                                                        }
                                                    }
                                                    if !found {
                                                        return quote_spanned!{value.0.span()=> compile_error!("Unrecognized value name.");}.into();
                                                    }
                                                }
                                                lit => {
                                                    return quote_spanned! { lit.span()=> compile_error!("Invalid data type.");}.into();
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => return e,
                                }
                            } else {
                                return quote_spanned! { attr.path.span()=> compile_error!("'input' macro attribute must be a list, e.g: #[input(name = \"x\", type=\"text\")].");}.into();
                            }
                        }
                        else if attr.path.is_ident("label") {
                            let meta = attr.parse_meta().unwrap();

                            if let syn::Meta::NameValue(name_val) = &meta {
                                if let syn::Lit::Str(val) = &name_val.lit {
                                    label = Some(val.value());
                                }
                                else {
                                    return quote_spanned! {name_val.lit.span()=> compile_error!("Value must be a str.");}.into();
                                }
                            }
                            else {
                                return quote_spanned! {meta.span()=> compile_error!("Must be a name value e.g: label = \"Username:\"");}.into();
                            }

                        }
                        else {
                            return quote_spanned! {attr.path.span()=> compile_error!("Unrecognized attribute name.");}.into();
                        }
                    }

                    if !name_added {
                        input_attributes.push(format!("name=\"{}\"", input_name));
                    }

                    if !id_added {
                        input_attributes.push(format!("id=\"{}\"", input_name));
                    }

                    let mut inp = String::from("\t<input ");

                    for attr in &input_attributes {
                        inp.push_str(attr);
                        inp.push_str(" ");
                    }

                    inp.push_str(">\n");

                    if let Some(label) = label {
                        let label = format!("\t<label for =\"{}\">{}</label>\n", input_id, label);
                        inputs.push(label);
                    }
                    else {
                        let label = input_name.to_title_case();
                        let label = format!("\t<label for =\"{}\">{}:</label>\n", input_id, label);
                        inputs.push(label);
                    }
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
