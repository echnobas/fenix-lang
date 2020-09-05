mod scanner;
mod lexer_types;

#[macro_use]
extern crate lazy_static;

fn main() {
	let mut scanner = scanner::scanner::new("(){}\n// hello world lmao\n (){} \"hello\" 1234.123 if ({{}})".to_string());
	scanner.scan_tokens();
	println!("{:#?}", scanner.get_tokens());
}
