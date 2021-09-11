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
		println!("TOKEN : '{}' ", token);
		if self.reserved_symbol_lookup.contains_key(token){
			let op = self.reserved_symbol_lookup.get(token).unwrap();
			println!("Identified reserved word: '{}' ", op.print());
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
