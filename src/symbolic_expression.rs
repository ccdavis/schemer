
use crate::primitives::Cell;
use crate::list::List;
use crate::interpreter::Environment;



#[derive(Clone)]
pub enum SExpression{
	List(List),
	Cell(Cell),
	Null,	
}



impl SExpression{
	
	pub fn print(&self)->String{
		match &self{
			&SExpression::Cell(cell) => cell.print(),
			&SExpression::List(list) => list.print(),
			SExpression::Null => String::from("Null"),
		}		
	}
	
	pub fn eval_as_bool(self, envr:&Environment) -> Result<Cell,String>{
		match self{
			SExpression::Cell(cell) => {
				cell.eval_as_bool().clone()
			},
			SExpression::List(_) =>{
				match envr.evaluate(self){
					Ok(value)=> value.eval_as_bool(&envr).clone(),
					Err(message) => Err(message),
				}
					
			},
			SExpression::Null => Err("Null is not a boolean".to_string()),
		}
	}
	// A helper to extract the simple Rust bool value while bringing along any type errors
	pub fn eval_as_rust_bool(self, envr:&Environment) -> Result<bool, String>{
		match self.eval_as_bool(&envr) {
			Ok(bool_value) =>{
				match bool_value{
					Cell::Bool(truth) =>  Ok(truth),
					_ => Err(format!("Not a boolean type {}",bool_value.print())),
				}
			},
			Err(message) =>  Err(message),
		}
	}
	
	// Special version of the more general evaluate
	pub fn eval_as_number(self, envr:&Environment)->Result<Cell, String>{
		match self{
			SExpression::Cell(cell) => {
				cell.eval_as_number().clone()
			},
			SExpression::List(_) =>{
				match envr.evaluate(self){
					Ok(value)=> value.eval_as_number(&envr).clone(),
					Err(message) => Err(message),
				}
					
			},
			SExpression::Null => Err("Null is not a number".to_string()),
		}
	}
			
} // impl SExpression
