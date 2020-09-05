use std::io::Write;

#[macro_use]
extern crate lazy_static;

mod scanner;
mod lexer_types;



fn main() {
	loop {
		let code: String = input("> ").unwrap();
		match code.as_str() {
			"exit" => { std::process::exit(0) },
			"clear" => { std::process::Command::new("cmd.exe").args(&["/c", "cls"]).status().unwrap().success(); },
			_ => {
				let mut scanner = scanner::scanner::new(code);
				scanner.scan_tokens();
				println!("{:#?}", scanner.get_tokens());
			}
		}
	}
}

fn input(prompt: &str) -> std::io::Result<String> {
	let mut result: String = String::new();
	print!("{}", prompt);
	std::io::stdout().flush()?;
	std::io::stdin().read_line(&mut result)?;
	Ok(result.trim().to_string())
}