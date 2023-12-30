# 4 billion if statements

per https://andreasjhkarlsson.github.io//jekyll/update/2023/12/27/4-billion-if-statements.html

Generate brute if-statement for determining even/odd of u32. Careful to use as it consumes large disk space.

```
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

if_stmts_gen!(429467295); // Change the number here as needed
f(num);
```


Code is generated by procedural macro that expands like this


```
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use four_billion_if_stmts_macro::if_stmts_gen;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        {
            ::std::io::_eprint(
                format_args!("Please provide a number as an argument.\n"),
            );
        };
        return;
    }
    let num = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            {
                ::std::io::_eprint(format_args!("Please provide a valid number.\n"));
            };
            return;
        }
    };
    fn f(num: u32) {
        if num == 0u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 1u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 2u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 3u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 4u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 5u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 6u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 7u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 8u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 9u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 10u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 11u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 12u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 13u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 14u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 15u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 16u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 17u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 18u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 19u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 20u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 21u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 22u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 23u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 24u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 25u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 26u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 27u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 28u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 29u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 30u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 31u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 32u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 33u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 34u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 35u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 36u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 37u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 38u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 39u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 40u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
            };
        } else if num == 41u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "odd"));
            };
        } else if num == 42u32 {
            {
                ::std::io::_print(format_args!("{0}\n", "even"));
        ...
```
