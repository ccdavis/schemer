
use crate::primitives::Cell;
use crate::primitives::NumericOperator;
use crate::primitives::LogicalOperator;
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


	fn subtract(list:List)->Result<SExpression,String>{			
		subtract_from(Cell::Int(0), list)
	}
	
	fn subtract_from(left_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(){
			Err(message)=>{
				Err(message)
			},
			Ok(right_value) =>{			
				
				let partial_sum = match (left_value, right_value) {			
					(Cell::Int(l),Cell::Int(r)) => Cell::Int(l - r),
					(Cell::Int(l), Cell::Flt(r)) => Cell::Flt(l as f64 - r),
					(Cell::Flt(l), Cell::Int(r)) => Cell::Flt(l - r as f64),
					(Cell::Flt(l), Cell::Flt(r)) => Cell::Flt(l - r),
					_ => {				
						// Type error:
						panic!("Data type error. Type checking should have caught this earlier.");
					},
				}; // match					
				if list.rest().is_empty(){
					Ok(SExpression::Cell(partial_sum))
				}else{			
					subtract_from(partial_sum, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}



	fn multiply(list:List)->Result<SExpression,String>{			
		multiply_by(Cell::Int(1), list)
	}
	
	fn multiply_by(left_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(){
			Err(message)=>{
				Err(message)
			},
			Ok(right_value) =>{							
				let partial_product = match (left_value, right_value) {			
					(Cell::Int(l),Cell::Int(r)) => Cell::Int(l * r),
					(Cell::Int(l), Cell::Flt(r)) => Cell::Flt(l as f64 * r),
					(Cell::Flt(l), Cell::Int(r)) => Cell::Flt(l * r as f64),
					(Cell::Flt(l), Cell::Flt(r)) => Cell::Flt(l * r),
					_ => {				
						// Type error:
						panic!("Data type error. Type checking should have caught this earlier.");
					},
				}; // match					
				if list.rest().is_empty(){
					Ok(SExpression::Cell(partial_product))
				}else{			
					multiply_by(partial_product, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}



	fn divide(list:List)->Result<SExpression,String>{			
		// If there's only one argument to /
		if list.rest().is_empty(){
			divide_into(Cell::Int(1), list)		
		}else{
			match list.first().eval_as_number(){
				Ok(numerator) => divide_into(numerator, list.rest()),
				Err(message) => Err(message),
			} // match				
		} // else
	}
	
	fn divide_into(numerator_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(){
			Err(message)=>{
				Err(message)
			},
			Ok(denominator_value) =>{							
				let partial_product = match (numerator_value, denominator_value) {			
					(Cell::Int(n),Cell::Int(d)) => Cell::Int(n / d),
					(Cell::Int(n), Cell::Flt(d)) => Cell::Flt(n as f64 / d),
					(Cell::Flt(n), Cell::Int(d)) => Cell::Flt(n / d as f64),
					(Cell::Flt(n), Cell::Flt(d)) => Cell::Flt(n / d),
					_ => {				
						// Type error:
						panic!("Data type error. Type checking should have caught this earlier.");
					},
				}; // match					
				if list.rest().is_empty(){
					Ok(SExpression::Cell(partial_product))
				}else{			
					divide_into(partial_product, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}
	
	fn eval_or(list:List)->Result<SExpression, String>{
		// eval each element as true or false. If a number, 0 is false anything else is true
		Ok(SExpression::Cell(Cell::Bool(true)))
		
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
			Subtract=>subtract(list), 
			Multiply=> multiply(list),
			Divide=> divide(list),
			Modulo=> Err(not_implemented),
		}
	}
	

	// Assuming it is not a null list and we have an operator or function, pass its cdr in and apply it:
	pub fn apply_logical_operator(func:LogicalOperator, list:List)-> Result<SExpression, String>{		
		// The cdr (now list) must have at least two items
		if list.is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");			
		}
		
		if list.rest().is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");
					
		}
		
		use crate::primitives::LogicalOperator::*;
		let not_implemented = String::from("Operator not implemented");
		match func {
			Or=> eval_or(list),				
			//And=>eval_and(list), 
			//Not=> eval_not(list),
			//Xor=> eval_xor(list),
			Greater=> Err(not_implemented),
			_ =>Err(not_implemented),
		}
	}
	


