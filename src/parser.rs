use crate::primitives::*;
use crate::symbolic_expression::SExpression;
use crate::list::*;
use std::str::FromStr;
use std::num::ParseFloatError;
use std::collections::HashMap;


const TRACE:bool = false;

#[derive(Debug)]
pub enum ParseError {
  Reason(String),
}
#[derive(Clone,Debug)]
enum TokenType{
	StringLiteral(String),
	LeftParen,
	RightParen,
	Other(String),	
	EOF,
}
#[derive(Clone,Debug)]
pub struct Token{
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
	all_chars:Vec<char>,
}

impl Lexer{

	fn end_of_input(&self)->bool{
		self.pos == self.text.len()
	}

	pub fn new(text:String)->Self{
		Self{
			text:text.clone(),
			all_chars:text.chars().collect(),
			pos:0,
			line:1,
			column:0,
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
	
	fn this_char(&self)-> char {
		if self.end_of_input() {
			4 as char // control-d
		} else {
			self.all_chars[self.pos]
		}
	}
	
	fn whitespace(&self)->bool{
		self.this_char() == ' ' || 
		self.this_char() == '\t' || 
		self.this_char() == '\n' ||
		self.this_char() == '\r'
	}
	
	fn begin_comment(&self)->bool{
		self.this_char() == ';'
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
		c == '.' ||
		c == '?' ||
		c == '=' ||
		c == '<' ||
		c == '>' ||
		c == '!'
	}
	
	fn skip_whitespace(&mut self){		
		while !self.end_of_input() && self.whitespace(){
			self.advance();						
		}
	}
	
	fn skip_comment(&mut self){
		if self.begin_comment(){
			while self.this_char() !='\n'  &&  !self.end_of_input(){
				self.advance();
			}
		}
		if TRACE{println!("Skipped comments");}				
	}
	
	
	pub fn next(&mut self)->Token{		
		self.skip_whitespace();		
		self.skip_comment();
			
		if self.end_of_input() {
			return Token{token_type:TokenType::EOF,line:self.line,column:self.column};
		}
		
		if TRACE{println!("Look at {}",&self.this_char());}
				
		let next_token = match self.this_char(){		
			'(' => {
				self.advance();
				Token {token_type:TokenType::LeftParen, line:self.line, column:self.column}
			},
			')' => {
				self.advance();
				Token { token_type:TokenType::RightParen,line:self.line, column:self.column}
			},
			'"' =>{
				let starting_line = self.line;
				let starting_column = self.column;
				
				self.advance(); // eat first "
				let mut content =  "".to_string();
				while self.this_char() != '"'{
					content.push(self.this_char());
					self.advance();
				}
				
				self.advance(); // eat the second " 
				Token{token_type:TokenType::StringLiteral(content), line:starting_line, column:starting_column}				
			},
			_ => {
				if TRACE{println!("unclassified token...");}
				if self.symbol_or_number_char() {
					if TRACE{println!("symbol or number...");}
					let mut content = "".to_string();
					while self.symbol_or_number_char(){
						content.push(self.this_char());
						self.advance();
					}
					Token{token_type:TokenType::Other(content),line:self.line,column:self.column}
				}else{
					// Something we didn't account for
					// TODO: throw a real error!
					panic!("Can't handle character '{}' at {}, {}",self.this_char(),self.line, self.column);										
				}
				
			},			
		};				
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

pub fn tokenize(text:String)->Vec<Token>{
	let mut lex = Lexer::new(text);
	let mut all_tokens = Vec::new();
	if TRACE {println!("Started lexing...");}
	while !lex.end_of_input(){
		let t = lex.next();
		if TRACE{println!("Token: {:?}",&t);}
		all_tokens.push(t);
	}
	all_tokens	
}

pub struct Parser{
	reserved_symbol_lookup:HashMap<String, Cell>,	
}

impl Parser{

	pub fn new()->Self{
		Self{ reserved_symbol_lookup:map_cell_from_string()}
	}
	
	pub fn parse_tokens<'a>(&self, tokens:&'a [Token])->Result<(SExpression, &'a [Token]), ParseError> {				
	  let (token, rest) = tokens.split_first()
		.ok_or(
		  ParseError::Reason("could not get token".to_string())
		)?;
		
	  match &token.token_type {
		 TokenType::LeftParen=> self.read_list_tokens(rest),
		 TokenType::RightParen => Err(ParseError::Reason("unexpected `)`".to_string())),
		_ => Ok((self.parse_cell_token(token), rest)),
	  }
	}
	
	fn read_list_tokens<'a>(&self, tokens: &'a [Token]) -> Result<(SExpression, &'a [Token]), ParseError> {
	  let mut res: Vec<SExpression> = vec![];
	  let mut xs = tokens;
	  loop {
	  
		let (next_token, rest) = xs
		  .split_first()
		  .ok_or(ParseError::Reason("could not find closing `)`".to_string()))?;
		  
		match next_token.token_type{
			TokenType::RightParen=>{
				return Ok((SExpression::List(List::make_from_sexps(res)), rest)) // skip `)`, head to the token after
			},
			TokenType::EOF=> return Err(ParseError::Reason("could not find closing `)` and reached end of input.".to_string())),
			_=>{
				let (exp, new_xs) = self.parse_tokens(&xs)?;
				res.push(exp);
				xs = new_xs;
			}
		} // match
	  } // loop
	}
	
	
	fn parse_cell_token(&self, token: &Token) -> SExpression{
		//println!("TOKEN : '{}' ", token);
		let (token_data,is_string) = match token.token_type{
			TokenType::Other(ref value)=>(value,false),
			TokenType::StringLiteral(ref value)=> (value,true),
			_ => panic!("Unexpected token type at {}, {}",token.line,token.column),
		};
		
		if !is_string && self.reserved_symbol_lookup.contains_key(token_data.as_str()){
			let op = self.reserved_symbol_lookup.get(token_data.as_str()).unwrap();
//			println!("Identified reserved word: '{}' ", op.print());
			SExpression::Cell(op.clone())		
		}else{
			if is_string{
				SExpression::Cell(Cell::Str(token_data.to_string()))
		// it should parse as a number or else it's a symbol; built-in operators and strings are already eliminated 
			}else{	
				let potential_float: Result<f64, ParseFloatError> = token_data.parse();
				match potential_float {
					Ok(v) => SExpression::Cell(Cell::Flt(v)),
					Err(_) => SExpression::Cell(Cell::Symbol(0, token_data.to_string().clone()))
				}
			}			
		}			
	}

	
} // impl Parser
