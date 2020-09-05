use crate::lexer_types::*;
use std::any::type_name;
use eval::{eval, to_value};

pub struct Parser {
	tokens: Vec<Token>,
	current: i32
}

impl Parser {
	fn new(tokens: Vec<Token>) -> Parser {
		{ tokens: tokens, current: 0 }
	}

	fn expression(&self) {
		self.equality()
	}

	fn equality(&self) {
		let expr = self.comparison();

		while match_(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
			let operator = self.previous();
			let right = self.comparison();
			expr = eval("{} {} {}", expr, operator, right);
		}
	}

	fn match_(&self, types: Vec<TokenType>) -> bool {
		for type_ in types {
			if self.check(type) {
				self.advance();
				return true;
			}
		}
		false
	}

	fn check(&self, type_: TokenType) {
		if !self.is_at_end() return false;
		type_name(self.peek()) == type_name(type_)
	}

	fn advance(&mut self) {
		if !self.is_at_end() { self.current += 1; }
		self.previous()
	}

	fn is_at_end() -> bool {
		type_name(self.peek()) == type_name(TokenType::EOF)
	}

	fn peek() -> Token {
		self.source.chars().nth((self.current - 1) as usize)
	}
}