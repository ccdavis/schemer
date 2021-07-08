
use crate::primitives::Cell;
use crate::primitives::NumericOperator;
use crate::symbolic_expression::SExpression;
use crate::list::List;


	fn add(list:List)->Result<SExpression,String>{			
		add_to(Cell::Int(0), list)
	}
	
	fn add_to(left_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(){
			Err(message)=>{
				Err(message)
			},
			Ok(right_value) =>{			
				
				let partial_sum = match (left_value, right_value) {			
					(Cell::Int(l),Cell::Int(r)) => Cell::Int(l + r),
					(Cell::Int(l), Cell::Flt(r)) => Cell::Flt(l as f64 + r),
					(Cell::Flt(l), Cell::Int(r)) => Cell::Flt(l + r as f64),
					(Cell::Flt(l), Cell::Flt(r)) => Cell::Flt(l + r),
					_ => {				
						// Type error:
						panic!("Data type error. Type checking should have caught this earlier.");
					},
				}; // match					
				if list.rest().is_empty(){
					Ok(SExpression::Cell(partial_sum))
				}else{			
					add_to(partial_sum, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}
		
// Evaluate any  S-Expression
pub fn evaluate(exp:SExpression)-> Result<SExpression,String>{		
	match exp{
		SExpression::Cell(_)=>Ok(exp),
		SExpression::List(list)=> list.evaluate(),
		SExpression::Null => Ok(exp)
			
	}
}

	// Assuming it is not a null list and we have an operator or function, pass its cdr in and apply it:
	pub fn apply_operator(func:NumericOperator, list:List)-> Result<SExpression, String>{		
		// The cdr (now list) must have at least two items
		if list.is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");			
		}
		
		if list.rest().is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");
					
		}
		
		use crate::primitives::NumericOperator::*;
		let not_implemented = String::from("Operator not implemented");
		match func {
			Add=> add(list),				
			Subtract=>Err(not_implemented),
			Multiply=> Err(not_implemented),
			Divide=> Err(not_implemented),
			Modulo=> Err(not_implemented),
		}
	}
	

