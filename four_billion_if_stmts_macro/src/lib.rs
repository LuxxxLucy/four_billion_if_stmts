extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitInt};

#[proc_macro]
pub fn if_stmts_gen(input: TokenStream) -> TokenStream {
    let times = parse_macro_input!(input as LitInt).base10_parse::<u32>().unwrap();

    let ifs = (0..times).map(|i| {
        let output = if i % 2 == 0 { "even" } else { "odd" };
        if i == 0 {
            quote! { if num == #i { println!("{}", #output); } }
        } else {
            quote! { else if num == #i { println!("{}", #output); } }
        }
    });

    let expanded = quote! {
        fn f(num: u32) {
            #( #ifs )*
            else { println!("Number out of range (0-{})", #times - 1); }
        }
    };

    TokenStream::from(expanded)
}

