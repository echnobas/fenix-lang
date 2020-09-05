#[macro_use]
extern crate lazy_static;

use std::io::Write;

mod scanner;
mod lexer_types;



fn main() {
	loop {
		let mut scanner = scanner::scanner::new(input("> ").unwrap());
		scanner.scan_tokens();
		println!("{:#?}", scanner.get_tokens());
	}
}

fn input(prompt: &str) -> std::io::Result<String> {
	let mut result: String = String::new();
	print!("{}", prompt);
	std::io::stdout().flush()?;
	std::io::stdin().read_line(&mut result)?;
	Ok(result.trim().to_string())
}