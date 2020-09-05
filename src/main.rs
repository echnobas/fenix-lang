#[macro_use]
extern crate lazy_static;

use std::io::Write;

mod scanner;
mod lexer_types;



fn main() {
	let mut scanner = scanner::scanner::new("".to_string());
	scanner.scan_tokens();
	println!("{:#?}", scanner.get_tokens());
}