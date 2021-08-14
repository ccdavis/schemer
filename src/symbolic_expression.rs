
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

	pub fn as_number(self)->Result<Cell,String> {
		match self{
			SExpression::Cell(cell)=>cell.eval_as_number(),				
			_=>Err(String::from("Not a number type")),
		}
	}
	
	pub fn as_bool(self)->Result<Cell,String> {
		match self{
			SExpression::Cell( cell)=>cell.eval_as_bool(),				
			_=>Err(String::from("Not a boolean type")),
		}
	}
	
	pub fn as_rust_bool(self)->Result<bool,String>{
		match self.as_bool(){
			Ok(bool_value)=>{
				match bool_value{
					Cell::Bool(truth) =>  Ok(truth),
					_ => Err(format!("Not a boolean type {}",bool_value.print())),				
				}
			},
			Err(message) =>  Err(message),			
		}
	}
	
	
} // impl SExpression
