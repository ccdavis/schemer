
use crate::primitives::Cell;
use crate::primitives::NumericOperator;
use crate::primitives::LogicalOperator;
use crate::primitives::SpecialForm;
use crate::symbolic_expression::SExpression;
use crate::list::List;


// Built in simple functions
use std::collections::HashMap;
// Results of 'define' go here
pub struct Environment{
	pub definitions_by_symbol:HashMap<String, usize>,	
	definitions:Vec<SExpression>
}



/*
High-performance storage of all defined symbols:

Q: Why doesn't the compiler and borrow-checker in particular allow for this?

		definitions: Vec<Sexpression>,
		definitions_by_name:HashMap<s_name:String, &Sexpression>,
	...
	
		definitions.push(exp);
		let last = definitions.len()-1;
		definitions_by_name.insert(name, &definitions[last]);
	
A:
	The Vec may get resized as it gets added to; the memory locations of the references aren't fixed. In C++ you could do this safely either
	with a pre-reserved vector or better yet with a deque which has the guarantee of no copying and moving of contents; it is made up of a sequence of linked blocks (you give up a bit of lookup time perf in exchange.)

Q: Why doesn't the reverse work: Storing references in the Vec and the original in the HashMap?

A: Either the borrow-checker isn't smart enough to automatically assign lifetimes or the HashMap cannot guarantee  no moving around of its
content...  I couldn't figure out a way to specify lifetimes to make it compile: From a C++ point-of view this should be 
safe but it may well not be in Rust.
//

The cannonical solution to this in Rust seems to be to store indexes into the Vec  in the HashMap rather than direct references
to the same data. It feels a bit less direct or performant but certainly safer.

The other solution would be to use Rc<>  in both containers.

*/
impl  Environment{
	pub fn new()->Self{
		let mut no_definitions :HashMap<String, usize> = HashMap::new();
		let mut empty_symbol_table = Vec::new();
		Environment{ definitions_by_symbol: no_definitions, definitions:empty_symbol_table}
	}
	
	
	// Shortcut to add symbols to the environment
	pub fn define(mut self, name:String, value:SExpression)->Result<i32, String>{
		if self.definitions_by_symbol.contains_key(&name){
			Err(format!("{} already defined.", &name))
		}else{
			let number = self.definitions.len();
			self.definitions_by_symbol.insert(name,number); 
			self.definitions.push(value);
			Ok(number as i32)
			}								
	}
	
	pub fn get_definition_by_symbol(&self, s:String)-> Result<SExpression,String>{
		match self.definitions_by_symbol.get(&s) {
			Some(number) => {				
				Ok(self.definitions[*number].clone())
			},
			_ => Err(format!("Symbol {} not defined.",&s))
		}											
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
						self.get_definition_by_symbol(symbol)
						
					},
					_ => Ok(SExpression::Cell(c)),
				},
			SExpression::List(list)=> list.evaluate(&self),
			SExpression::Null => Ok(exp)
				
		}
	}
		
		
	pub fn apply_special_form(&self, func:SpecialForm, args:List)->Result<SExpression,String>{
		match func {
			SpecialForm::Define => {
				let new_symbol = args.first();
				let value_for_symbol = args.rest().first();
				println!("Would define {} as {}", new_symbol.print(), value_for_symbol.print());
				Ok(*new_symbol)								
			},
			_ => Err(format!("Special form {} not implemented!", "not printable").to_string())
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


