mod json_schema;
use json_schema::{StructsTemplate, get_string_literal};
use proc_macro::TokenStream;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    println!("{:#?}", input);
    "fn hello() {println!(\"hello world\")}".parse().unwrap()
}

#[proc_macro]
pub fn generate(input: TokenStream) -> TokenStream {
    let filename = get_string_literal(input).unwrap();
    let result = StructsTemplate::render(&filename).unwrap();
    match result.parse() {
        Ok(res) => {
            println!("res: {:#?}", res);
            res
        },
        Err(e) => {
            println!("error: {:#?}", e);
            TokenStream::default()
        },
    }
}