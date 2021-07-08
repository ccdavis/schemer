
use crate::primitives::Cell;
use crate::list::List;
use crate::interpreter::evaluate;


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
	
	// Special version of the more general evaluate
	pub fn eval_as_number(self)->Result<Cell, String>{
		match self{
			SExpression::Cell(cell) => {
				cell.eval_as_number().clone()
			},
			SExpression::List(_) =>{
				match evaluate(self){
					Ok(value)=> value.eval_as_number().clone(),
					Err(message) => Err(message),
				}
					
			},
			SExpression::Null => Err("Null is not a number".to_string()),
		}
	}
			
} // impl SExpression
