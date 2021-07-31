
use crate::primitives::Cell;
use crate::primitives::NumericOperator;
use crate::primitives::LogicalOperator;
use crate::symbolic_expression::SExpression;
use crate::list::List;


// Built in simple functions
use std::collections::HashMap;
// Results of 'define' go here
pub struct Environment{
	pub definitions:HashMap<String,SExpression>,	
}

impl Environment{
	pub fn new()->Self{
		let mut no_definitions :HashMap<String, SExpression> = HashMap::new();
		Environment{ definitions: no_definitions}
	}



	fn add(&self, list:List)->Result<SExpression,String>{			
		self.add_to(Cell::Int(0), list)
	}
	
	fn add_to(&self, left_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(&self){
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
					self.add_to(partial_sum, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}


	fn subtract(&self, list:List)->Result<SExpression,String>{			
		self.subtract_from(Cell::Int(0), list)
	}
	
	fn subtract_from(&self, left_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(&self){
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
					self.subtract_from(partial_sum, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}



	fn multiply(&self, list:List)->Result<SExpression,String>{			
		self.multiply_by(Cell::Int(1), list)
	}
	
	fn multiply_by(&self, left_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(&self){
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
					self.multiply_by(partial_product, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}



	fn divide(&self, list:List)->Result<SExpression,String>{			
		// If there's only one argument to /
		if list.rest().is_empty(){
			self.divide_into(Cell::Int(1), list)		
		}else{
			match list.first().eval_as_number(&self){
				Ok(numerator) => self.divide_into(numerator, list.rest()),
				Err(message) => Err(message),
			} // match				
		} // else
	}
	
	fn divide_into(&self, numerator_value:Cell, list:List)-> Result<SExpression, String>{		
		match list.first().eval_as_number(&self){
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
					self.divide_into(partial_product, list.rest())		
				}
			} // Some(right_value)
		} // match					
	}
	
	fn eval_greater(&self, list:List)->Result<SExpression,String>{
		if list.rest().is_empty(){
			return Ok(SExpression::Cell(Cell::Bool(true)));
		}
		let left  = list.first().eval_as_number(&self);
		let right = list.rest().first().eval_as_number(&self);
		
		let gt = match (left.unwrap(),right.unwrap()) {
			(Cell::Int(i),Cell::Int(j)) => i > j,
			(Cell::Int(i), Cell::Flt(j))=> i as f64 > j,
			(Cell::Flt(i), Cell::Int(j)) => i > j as f64,
			(Cell::Flt(i), Cell::Flt(j)) => i > j,
			_ => panic!("Type checking error, needed number type."),
		};
			
		if  gt {
			self.eval_greater(list.rest())
		}else{
			Ok(SExpression::Cell(Cell::Bool(false)))
		}	
	}	
	
	fn eval_or(&self, list:List)->Result<SExpression, String>{
		let bool_value  = list.first().eval_as_rust_bool(&self);
		match bool_value {
			Ok(truth)=>{
				if truth { 
					Ok(SExpression::Cell(Cell::Bool(true)))
				}else{
					if list.rest().is_empty(){
						Ok(SExpression::Cell(Cell::Bool(true)))
					}else{
						self.eval_or(list.rest())				
					}
						}
				}, // ok truth
				Err(message)=> Err(message)		
			} // match bool_value
					
	}

		
	// Evaluate any  S-Expression
	pub fn evaluate(&self, exp:SExpression)-> Result<SExpression,String>{		
		match exp{
			SExpression::Cell(c)=>
				match c {				
					// The idea is to  use the number instead of the name to do
					// lookup in a vector of definitions for better performance...
					// but the hash map gets us started.
					Cell::Symbol(number, symbol)=> {
						match self.definitions.get(&symbol) {
							Some(expr) => Ok(expr.clone()),
							_ => Err(format!("Symbol {} not defined.",&symbol))
						}
						
						
					},
					_ => Ok(SExpression::Cell(c)),
				},
			SExpression::List(list)=> list.evaluate(&self),
			SExpression::Null => Ok(exp)
				
		}
	}
	

	// Assuming it is not a null list and we have an operator or function, pass its cdr in and apply it:
	pub fn apply_operator(&self, func:NumericOperator, list:List)-> Result<SExpression, String>{		
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
			Add=> self.add(list),				
			Subtract=> self.subtract(list), 
			Multiply=>  self.multiply(list),
			Divide=>  self.divide(list),
			Modulo=> Err(not_implemented),
		}
	}
	

	// Assuming it is not a null list and we have an operator or function, pass its cdr in and apply it:
	pub fn apply_logical_operator(&self, func:LogicalOperator, list:List)-> Result<SExpression, String>{		
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
			Greater=> self.eval_greater(list),
			Or=> self.eval_or(list),				
			//And=>eval_and(list), 
			//Not=> eval_not(list),
			//Xor=> eval_xor(list),
			Greater=> Err(not_implemented),
			_ =>Err(not_implemented),
		}
	}
	} // Environment


