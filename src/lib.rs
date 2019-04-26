extern crate proc_macro;

use heck::TitleCase;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro_derive(ErrorGen)]
pub fn error_gen_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ErrorGenInput);
    input.codegen().into()
}

struct ErrorGenInput {
    root: syn::DeriveInput,
}

impl Parse for ErrorGenInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(ErrorGenInput::new)
    }
}

impl ErrorGenInput {
    fn new(root: syn::DeriveInput) -> Self {
        ErrorGenInput { root }
    }

    fn codegen(self) -> proc_macro2::TokenStream {
        let name = &self.root.ident;
        let name_text = name.to_string().to_title_case().to_lowercase();
        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str(#name_text)
                }
            }

            impl std::error::Error for #name {}
        }
    }
}
