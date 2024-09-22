use std::env;
use std::fs::read_to_string;

use interpreter::interpret;

mod ast;
mod interpreter;
mod parser;
mod tokenizer;

fn main() {
    let mut args = env::args();
    let Some(input_file) = args.nth(1) else {
        show_usage();
        return;
    };
    let result = read_to_string(input_file);
    match result {
        Result::Err(err) => {
            println!("Failed to load input file with error {err:?}")
        }
        Result::Ok(source) => interpret(&source),
    }
}

fn show_usage() {
    println!(
        r"Usage:
        basic-like <script>

        <script> input file name with source code"
    );
}
