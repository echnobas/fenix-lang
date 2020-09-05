mod scanner;
mod lexer_types;

#[macro_use]
extern crate lazy_static;
extern crate wfd;

use std::env;
use std::fs;

use wfd::{DialogParams, FOS_ALLOWMULTISELECT, DialogError};

fn run_code(code: &str) {
	let mut scanner = scanner::scanner::new(code.to_string());
	scanner.scan_tokens();

	println!("{:#?}", scanner.get_tokens());
}

fn main() {
	let params = DialogParams {
		title: "Select an FenixLang File to open",
		file_types: vec![("FenixLang Files", "*.fnxl")],
		file_type_index: 2,
		default_extension: "fnxl",
		options: FOS_ALLOWMULTISELECT,
		..Default::default()
	};

	match wfd::open_dialog(params) {
		Ok(result) => {
			for file in result.selected_file_paths {
				let content = fs::read_to_string(file.to_str().unwrap())
					.expect("Something went wrong reading the file.");

				run_code(&content);
			}
		}

		Err(error) => match error {
			DialogError::HResultFailed { hresult, error_method } => {}
			DialogError::UserCancelled => {}
		}
	}
}