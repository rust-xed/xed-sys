use std::env;
use std::fs::File;
use std::io::{Read, Write};

use quote::quote;
use syn::parse_quote;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let input_file = args
        .get(1)
        .expect("USAGE: remove-methods <source-file> <dest-file>");
    let output_file = args
        .get(2)
        .expect("USAGE: remove-methods <source-file> <dest-file>");

    let mut input_file =
        File::open(&input_file).expect(&format!("Unable to open file {}", input_file));
    let mut output_file =
        File::create(&output_file).expect(&format!("Unable to open file {}", output_file));

    let mut src = String::new();
    input_file
        .read_to_string(&mut src)
        .expect("Unable to read source file");

    let syntax = syn::parse_file(&src).expect("Input file contained unparseable rust tokens");

    for item in syntax.items {
        match item {
            syn::Item::Fn(mut func) => {
                func.attrs = vec![parse_quote! { #[inline] }];

                let tokens = quote! { #func };

                writeln!(output_file, "{}", tokens).expect("Failed to write to output file");
            }
            _ => (),
        }
    }
}
