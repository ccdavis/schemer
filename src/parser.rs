use crate::primitives::*;
use crate::symbolic_expression::SExpression;
use crate::list::*;
use std::str::FromStr;
use std::num::ParseFloatError;
use std::collections::HashMap;

// I adapted the parsing code from this post:
//https://stopa.io/post/222
//
// The changes are only superficial, to make use of my more complex primitives and list constructors.

#[derive(Debug)]
pub enum ParseError {
  Reason(String),
}

enum TokenType{
	StringLiteral(String),
	LeftParen,
	RightParen,
	Other(String),	
	EOF,
}

struct Token{
	token_type:TokenType,
	line:usize,
	column:usize,	
}


// maintain the state of the lexical analysis plus keep track of column and line
pub struct Lexer{
	pos:usize,
	line:usize,
	column:usize,
	text:String,	
}

impl Lexer{

	fn end_of_input(&self)->bool{
		self.pos == self.text.len()
	}

	fn new(text:String)->Self{
		Self{
			text:text,
			pos:0,
			line:1,
			column:1,
		}
	}
	
	fn advance(&mut self){
		if !self.end_of_input(){
			self.pos += 1;
			if self.this_char() == '\n'{ 
				self.line += 1;
				self.column = 1;
			}else{
				self.column += 1;
			}						
		}
	}
	
	fn this_char(&self)->char{
		self.text[self.pos]
	}
	
	fn whitespace(&self)->bool{
		self.this_char() == ' ' || self.this_char() == '\t'
	}
	
	fn begin_comment(&self)->bool{
		self.this_char() == ";"
	}
	
	fn symbol_or_number_char(&self)->bool{
		let c = self.this_char();
		c.is_alphanumeric() || 
		c == '+' || 
		c == '-' || 
		c == '/' || 
		c == '*' || 
		c == '%' ||
		c == '\'' ||
		c == '!'
	}
	
	fn skip_whitespace(&mut self){		
		while !self.end_of_input() && self.whitespace(){
			self.advance();						
		}
	}

	fn next(&mut self)->Token{
		if self.begin_comment(){
			while self.this_char() !='\n'  &&  !self.end_of_input(){
				self.advance();
			}
		}
		
		self.skip_whitespace();
		if self.end_of_input(){
			return Token{token_type:EOF,line:self.line,column:self.column};
		}
				
		let next_token = match self.this_char(){		
			'(' =>{
				self.advance();
				Token {token_type:TokenType::LeftParen, line:self.line, column:self.column}
			},
			')' =>{
				self.advance();
				Token { token_type:TokenType::RightParen,line:self.line, column:self.column};
			},
			'"' =>{
				let starting_line = self.line;
				let starting_column = self.column;
				
				self.advance(); // eat first "
				let mut content =  "".to_string();
				while self.this_char()!='"'{
					content.push(self.this_char());
				}
				
				self.advance(); // eat the second " 
				Token{token_type:StringLiteral(content), line:starting_line, column:starting_column}				
			},
			_=> {
				if self.symbol_or_number_char() {
					let mut content = "".to_string();
					while self.symbol_or_number_char(){
						content.push(self.this_char());
					}
					Token{token_type:Other(content),line:self.line,column:self.column}
				}else{
					// Something we didn't account for
					// TODO: throw a real error!
					panic!("Can't handle character '{}' at {}, {}",self.this_char(),self.line, self.column);										
				}
				
			},			
		};
		self.advance();
		
		next_token
	}

}

pub fn lex(expression: String) -> Vec<String> {
// Ensure ( and ) are surrounded by whitespace, then
// organize items into strings
  expression
	.replace("(", " ( ")
	.replace(")", " ) ")
	.split_whitespace()
	.map(|x| x.to_string())
	.collect()
}


pub struct Parser{
	reserved_symbol_lookup:HashMap<String, Cell>,	
}

impl Parser{

	pub fn new()->Self{
		Self{ reserved_symbol_lookup:map_cell_from_string()}
	}
	
	
	pub fn parse<'a>(&self, tokens: &'a [String]) -> Result<(SExpression, &'a [String]), ParseError> {				
	  let (token, rest) = tokens.split_first()
		.ok_or(
		  ParseError::Reason("could not get token".to_string())
		)?;
	  match &token[..] {
		"(" => self.read_list(rest),
		")" => Err(ParseError::Reason("unexpected `)`".to_string())),
		_ => Ok((self.parse_cell(token), rest)),
	  }
	}

	fn read_list<'a>(&self, tokens: &'a [String]) -> Result<(SExpression, &'a [String]), ParseError> {
	  let mut res: Vec<SExpression> = vec![];
	  let mut xs = tokens;
	  loop {
		let (next_token, rest) = xs
		  .split_first()
		  .ok_or(ParseError::Reason("could not find closing `)`".to_string()))
		  ?;
		if next_token == ")" {	
		  return Ok((SExpression::List(List::make_from_sexps(res)), rest)) // skip `)`, head to the token after
		}
		let (exp, new_xs) = self.parse(&xs)?;
		res.push(exp);
		xs = new_xs;
	  }
	}

	fn parse_cell(&self, token: &str) -> SExpression{
		// This first block should handle (eventually) all built in symbols
		//println!("TOKEN : '{}' ", token);
		if self.reserved_symbol_lookup.contains_key(token){
			let op = self.reserved_symbol_lookup.get(token).unwrap();
//			println!("Identified reserved word: '{}' ", op.print());
			SExpression::Cell(op.clone())		
		}else{
			// This block will handle all primitive literals and symbols
			match token{
				//">" => SExpression::Cell(Cell::Op(NumericOperator::Add)),
				_ =>{
					let potential_float: Result<f64, ParseFloatError> = token.parse();
					match potential_float {
						Ok(v) => SExpression::Cell(Cell::Flt(v)),
						Err(_) => SExpression::Cell(Cell::Symbol(0, token.to_string().clone()))
					}
				}
			}
		}
			
	}

} // impl Parser
