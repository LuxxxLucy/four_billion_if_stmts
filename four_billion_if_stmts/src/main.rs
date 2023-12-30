use four_billion_if_stmts_macro::if_stmts_gen;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a number as an argument.");
        return;
    }

    let num = match args[1].parse::<u32>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("Please provide a valid number.");
            return;
        },
    };

    // the macro expands to a function f
    if_stmts_gen!(200); // Change the number here as needed
                        // change to 4294967295 if you want
    f(num);
}


