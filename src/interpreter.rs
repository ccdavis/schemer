use anyhow::{Context,Result};
use crate::primitives::Cell;
use crate::primitives::NumericOperator;
use crate::primitives::LogicalOperator;
use crate::primitives::SpecialForm;
use crate::symbolic_expression::SExpression;
use crate::list::List;
use std::rc::Rc;


use std::collections::HashMap;

static TRACE:bool = false;


// Built in simple functions
// Results of 'define' go here
#[derive(Clone)]
pub struct Environment<'a>{
	pub definitions_by_symbol:HashMap<String, usize>,	
	definitions:Vec<SExpression>,
	parent:Option<&'a Environment<'a>>,
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

The accepted solution to this in Rust seems to be to store indexes into the Vec  in the HashMap rather than direct references
to the same data. It feels a bit less direct or performant but certainly safer.

The other solution would be to use Rc<>  in both containers.

*/

	pub fn extend_environment<'a>(env:&'a mut Environment)->Environment<'a>{
		let mut no_definitions :HashMap<String, usize> = HashMap::new();
		let mut empty_symbol_table = Vec::new();
		Environment{ 
			parent : Some(env), 
			definitions_by_symbol: no_definitions, 
			definitions:empty_symbol_table}		
	}
	

impl  Environment <'_>{

	

	// A formatted list of all defined symbols in the environment (not including parent)
	pub fn print(&self)->String{
		let mut symbols = Vec::new();
		for (symbol, number) in &self.definitions_by_symbol{
			let expr = self.definitions[*number].clone();
			symbols.push(format!("{} : {} {}", &symbol, number, &expr.print()));			
		}
		symbols.join("\n")
	}
		
	pub fn new()->Self{
		let mut no_definitions :HashMap<String, usize> = HashMap::new();
		let mut empty_symbol_table = Vec::new();
		Environment{ 
			parent : None, 
			definitions_by_symbol: no_definitions, 
			definitions:empty_symbol_table}
	}
	

	pub fn make_child<'a>(&'a mut self)->Environment<'a>{
		let mut no_definitions :HashMap<String, usize> = HashMap::new();
		let mut empty_symbol_table = Vec::new();
		Environment{ 
			parent : Some(self), 
			definitions_by_symbol: no_definitions, 
			definitions:empty_symbol_table}		
	}
		
	
	// Shortcut to add symbols to the environment
	pub fn define(&mut self, name:String, value:SExpression)->Result<i32, String>{
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
			Some(number) => Ok(self.definitions[*number].clone()),			
			_ => {
				if let Some(outer) = self.parent{
					outer.get_definition_by_symbol(s)
				}else{
					Err(format!("Symbol {} not defined.",&s))
				}
			},
		}											
	}
	// extract  the cell version of an s-expression if it's a number type cell
	fn checked_number(item:Result<SExpression,String>)->Result<Cell,String>{
		match item{
			Ok(item)=>item.as_number(),
			Err(message)=>Err(message),
		}
	}
	
	fn checked_rust_bool(item:Result<SExpression,String>)->Result<bool,String>{
		match item{
			Ok(item)=>item.as_rust_bool(),
			Err(message)=>Err(message),
		}
	}
	
	fn add(&mut self, list:List)->Result<SExpression,String>{			
		self.add_to(Cell::Int(0), list)
	}
	
	fn add_to(&mut self, left_value:Cell, list:List)-> Result<SExpression, String>{				
		let next_item = self.evaluate(*list.first())?;		
		let right_value = next_item.as_number()?;	
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
	

	}


	fn subtract(&mut self, list:List)->Result<SExpression,String>{			
		let first_item = self.evaluate(*list.first())?;
		let leftmost_number = first_item.as_number()?;
		self.subtract_from(leftmost_number, list.rest())		
	}
	
	fn subtract_from(&mut self, left_value:Cell, list:List)-> Result<SExpression, String>{		
		let next_item = self.evaluate(*list.first())?;
		let right_value = next_item.as_number()?;
					
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

	}


	fn multiply(&mut self, list:List)->Result<SExpression,String>{			
		self.multiply_by(Cell::Int(1), list)
	}
	
	fn multiply_by(&mut self, left_value:Cell, list:List)-> Result<SExpression, String>{		
		let next_item = self.evaluate(*list.first())?;
		let right_value = next_item.as_number()?;
	
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
	
	}



	fn divide(&mut self, list:List)->Result<SExpression,String>{			
		// If there's only one argument to /
		if list.rest().is_empty(){
			self.divide_into(Cell::Int(1), list)		
		}else{
			let next_item = self.evaluate(*list.first())?;
			let numerator = next_item.as_number()?;
			self.divide_into(numerator, list.rest())			
		} // else
	}
	
	fn divide_into(&mut self, numerator_value:Cell, list:List)-> Result<SExpression, String>{		
		let next_item = self.evaluate(*list.first())?;
		let denominator_value = next_item.as_number()?;

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
	}
	
	fn eval_greater(&mut self, list:List)->Result<SExpression,String>{
		if list.rest().is_empty(){
			return Ok(SExpression::Cell(Cell::Bool(true)));
		}
		
		let left  = Environment::checked_number(self.evaluate(*list.first()))?;
		let right = Environment::checked_number(self.evaluate(*list.rest().first()))?;
		
		let gt = match (left,right){
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
	
	
	fn eval_less(&mut self, list:List)->Result<SExpression,String>{
		if list.rest().is_empty(){
			return Ok(SExpression::Cell(Cell::Bool(true)));
		}
		
		let left  = Environment::checked_number(self.evaluate(*list.first()))?;
		let right = Environment::checked_number(self.evaluate(*list.rest().first()))?;
		
		let lt = match (left,right){
			(Cell::Int(i),Cell::Int(j)) => i < j,
			(Cell::Int(i), Cell::Flt(j))=> (i as f64) < j,
			(Cell::Flt(i), Cell::Int(j)) => i < j as f64,
			(Cell::Flt(i), Cell::Flt(j)) => i < j,
			_ => panic!("Type checking error, needed number type."),
		};
			
		if  lt {
			self.eval_less(list.rest())
		}else{
			Ok(SExpression::Cell(Cell::Bool(false)))
		}	
	}	
	
	fn eval_equal(&mut self, list:List)->Result<SExpression,String>{
		if list.rest().is_empty(){
			return Ok(SExpression::Cell(Cell::Bool(true)));
		}
		
		let left  = Environment::checked_number(self.evaluate(*list.first()))?;
		let right = Environment::checked_number(self.evaluate(*list.rest().first()))?;
		
		let eq = match (left,right){
			(Cell::Int(i),Cell::Int(j)) => i == j,
			(Cell::Int(i), Cell::Flt(j))=> (i as f64) == j,
			(Cell::Flt(i), Cell::Int(j)) => i == j as f64,
			(Cell::Flt(i), Cell::Flt(j)) => i == j,
			_ => panic!("Type checking error, needed number type."),
		};
			
		if  eq {
			self.eval_equal(list.rest())
		}else{
			Ok(SExpression::Cell(Cell::Bool(false)))
		}	
	}	
	
		
	fn eval_or(&mut self, list:List)->Result<SExpression, String>{
		let truth = Environment::checked_rust_bool(self.evaluate(*list.first()))?;							
		if truth { 			
			Ok(SExpression::Cell(Cell::Bool(true)))
		}else{
			if list.rest().is_empty(){
				Ok(SExpression::Cell(Cell::Bool(true)))
			}else{
				self.eval_or(list.rest())				
			}
		}	
	}
	
	pub fn eval_and(&mut self, list:List)->Result<SExpression, String>{
		let truth = Environment::checked_rust_bool(self.evaluate(*list.first()))?;							
		if truth{
			if list.rest().is_empty() {
				Ok(SExpression::Cell(Cell::Bool(truth)))
			}else{
				self.eval_and(list.rest())							
			}
		}else{
			Ok(SExpression::Cell(Cell::Bool(truth)))
		}		
	}


		
	// Evaluate any  S-Expression
	pub fn evaluate(&mut self, exp:SExpression)-> Result<SExpression,String>{
		match exp{
			SExpression::Cell(c)=>
				match c {														
					// The idea is to  use the number instead of the name to do
					// lookup in a vector of definitions for better performance...
					// but the hash map gets us started.
					//
					// In the context where this is evaluated the symbol can only
					// refer to a variable; functions would be the first element
					// of a list which gets evaluated in the List::evaluate9)
					// function.
					Cell::Symbol(number, symbol)=> {
						if (TRACE) {println!("Try to evaluate symbol {}",&symbol);}
						self.get_definition_by_symbol(symbol)						
					},
					_ => Ok(SExpression::Cell(c)),
				},
			SExpression::List(list)=> list.evaluate(self),
			SExpression::Null => Ok(exp)
				
		}
	}
	
	//  Can take the formatted output in the result and direct to another stream
	fn evaluate_output(&mut self, args:List)->Result<SExpression,String>{
		let results = self.eval_each(args)?;
		let printed_results:String = results.iter()
			.map(|r| r.print())
			.collect::<Vec<String>>()
			.join(" ");
		
		println!("{}",&printed_results);
		Ok(SExpression::Cell(Cell::Str(printed_results)))				
	}
				
	pub fn apply_special_form(&mut self, func:SpecialForm, args:List)->Result<SExpression,String>{
		if TRACE{println!("Apply special form {}",&func.print());}
		match func {
			SpecialForm::If=> self.evaluate_if(args),		
			SpecialForm::Set=> self.evaluate_set(args),		
			SpecialForm::While=> self.evaluate_while(args),
			SpecialForm::Begin=>self.eval_each_return_last(args),
			SpecialForm::Output=> self.evaluate_output(args), 
			SpecialForm::Define => {
				let new_symbol = args.first();
				let value_for_symbol = args.rest().first();
				
				// Check for extra expressions after the one used as the value to define:
				if !args.rest().rest().is_empty(){
					return Err(format!("define can only take one expression as the value to assign to the symbol or lambda: {}",
						&new_symbol.print()));
				}
				
				match *new_symbol.clone(){
				// If it's a cell, it must be a symbol Cell::Symbol
					SExpression::Cell(cell)=>
						match cell{
							Cell::Symbol(number,name)=>{
								let evaluated_value = self.evaluate(*value_for_symbol)?;
								let def_id =self.define(name, evaluated_value)?;
								Ok(*new_symbol.clone())	
							},
							_=> Err(format!("Cannot re-define {}",&cell.print())),
						},										
										
					// If it's a list it must be the first part of a lambda
					// Split into arguments and body and type-check.
					SExpression::List(function_signature)=>{
						let parameter_names = function_signature.rest();												
						match *function_signature.first(){
							SExpression::Cell(n)=>
								match n{ // This *should* be the name of the function
									Cell::Symbol(number,name)=>{
										let params = Box::new(SExpression::List(parameter_names));
										let body = Box::new(*value_for_symbol);
										let value = SExpression::Cell(Cell::Lambda(params, body));
										let def_id = self.define(name, value);
										Ok(*new_symbol.clone())		
									},
									_ => Err(format!("Invalid function name: {}", &n.print())),
								},
							_ => Err(format!("Invalid function name: {}",&function_signature.first().print())),
						}																							
					},
					_=>Err(format!("Cannot apply special form treatment to {}",new_symbol.print())),										
				}
											
			},
			_ => Err(format!("Special form {} not implemented!", "not printable").to_string())
		}
	}
	
	// Requires three arguments: 'if' must have a test expression and both outcomes of the test.
	fn evaluate_if(&mut self, clauses:List)->Result<SExpression,String>{
		if clauses.is_empty(){
			return Err(format!("if expression must have three parts."));
		}
		
		let first_clause = *clauses.first();
		if TRACE{println!("Evaluate test expression in if:");}
		let test_result = self.evaluate(first_clause)?;			
		let truth = test_result.as_rust_bool()?;
			
		if truth{ // evaluate if branch
			if TRACE {println!("Evaluate if-true branch");}
			self.evaluate(*clauses.rest().first())
		}else{ // evaluate 'else' branch
			if TRACE{println!("Evaluate if-false branch");}
			self.evaluate(*clauses.rest().rest().first())
		}	
	}
	
	fn evaluate_while(&mut self, clauses:List)->Result<SExpression,String>{
		if clauses.is_empty(){
			return Err(format!("while expression must have two  clauses (test) (body)."));
		}
		
		if clauses.rest().is_empty(){
			return Err(format!("while expression must have two  clauses (test) (body)."));
		}
				
		let test_expression = *clauses.first();
		if TRACE{println!("test_expression: {:?}", &test_expression.print());}
		
		let mut test_result = self.evaluate(test_expression.clone())?;
		if TRACE{println!("test_result: {:?}", &test_result.print());}
		
		let mut truth = test_result.as_rust_bool()?;
		
		let mut return_value = SExpression::Cell(Cell::Bool(false));
		let body = *clauses.rest().first();
		if !clauses.rest().rest().is_empty(){
			return Err(format!("while expression body must not have  more than one s-expression."));
		}
		
		if TRACE{println!("body: {:?}",&body.print());}
				
		while truth{			
			return_value = self.evaluate(body.clone())?;
			
			if TRACE{println!("return value: {:?}",return_value.print());}
						
			test_result = self.evaluate(test_expression.clone())?;			
			if TRACE{println!("test_result: {:?}",test_result.print());}
			truth = test_result.as_rust_bool()?;
		}
		Ok(return_value)
	}
	
		
	// The result of set! will be the symbol number in the current scope. Mostly this
	// value is only a side-effect of set! which will be ignored.
	fn evaluate_set(&mut self, args:List)->Result<SExpression,String>{
		if args.is_empty(){
			return Err(format!("set! expression must have two arguments."));
		}
		
		let variable_to_change = *args.first();
		let change_to = args.rest();
		if change_to.is_empty(){
			return Err(format!("set! expression must have two arguments."));
		}
		
		if !change_to.rest().is_empty(){
			return Err(format!("set! expression must have only two arguments."));
		}
		
		let change_to_value = self.evaluate(*change_to.first())?;
		
		let (local_number,name) = match variable_to_change{
			SExpression::Cell(c)=>{
				match c{
					Cell::Symbol(n,s)=>Ok((n,s)),
					_=>Err(format!("First argument to set! must be a symbol but was {} instead.",c.print())),
				}
			},
			_=>Err(format!("First argument to set! must be a symbol.")),
		}?;
		
			
		let number = match self.definitions_by_symbol.get(&name){
			Some(n)=>Ok(*n),
			None=>Err(format!("{} not defined.", &name)),
		}?;
							
		self.definitions[number] = change_to_value;
		Ok(SExpression::Cell(Cell::Int(number as i64)))		
	}
	
	// Instead of evaluating the list as a whole, evaluate each s-expression
	// in the list and return a list of each of those evaluation results.
	// In other words, don't assume the list begins with a function name and the
	// rest is a set of arguments to it.
	//
	// This is like a (begin...) block but a special case for top-level code 
	// in a program file or from the REPL.
	pub fn eval_each(&mut self, args:List)->Result< Vec<SExpression>, String>{
		let mut eval_results:Vec<SExpression> = Vec::new();
		let mut remaining_args = args.clone();
		if TRACE {println!("In eval_each!");}
		while !remaining_args.is_empty(){
			if TRACE {println!("In eval_each with args: {}",&remaining_args.print());}
			let car = remaining_args.first();
			if TRACE{println!("Eval this expression: {}",car.print());}
			let value = self.evaluate(*car)?;			
			if TRACE {println!("In eval_each, got value: {}",&value.print());}
			eval_results.push(value);
			remaining_args = remaining_args.rest();
		}		
		Ok(eval_results)
	}
	
	// For bodies of lambdas or "code blocks" to allow multiple expressions. The last
	// expression is the value of the lambda or block.
	//
	// This is the behavior we want from a (begin ...) block of expressions.
	fn eval_each_return_last(&mut self,exprs:List)->Result<SExpression,String>{		
		let results = self.eval_each(exprs)?;		
		Ok(results[results.len()-1].clone())		
	}	
	
	// Assign all values to names in args
	fn define_all(&mut self, params:SExpression, values:Vec<SExpression>){
		let param_names = match params{
			SExpression::List(names)=>names,
			_=> panic!("Invalid parameter list {:?}",params.print()),
		};
		
		let mut remaining_names = param_names.clone();
		let mut arg_num = 0;
		while !remaining_names.is_empty(){
			let name = remaining_names.first();
			if arg_num+1 > values.len(){
				panic!("Mismatch between number of arguments and function parameters!");
			}
			let value = values[arg_num].clone();			
			//if TRACE {println!("Define {} as {}",&name, &value.print());}
			match *name{
				SExpression::Cell(Cell::Symbol(_,n))=>self.define(n,value),
				_=>panic!("A parameter name must be a symbol but you used {}",&*name.print()),
			};
			arg_num+=1;
			remaining_names = remaining_names.rest();
		}				
	}
		
	pub fn apply_function(&mut self, number:i32, name:String, args:List)->Result<SExpression, String>{
		if TRACE {println!("Try to evaluate symbol '{}' as function call",&name);}
		let func = self.get_definition_by_symbol(name)?;		
		if let SExpression::Cell(Cell::Lambda(params,body)) = func{
			// match the params to the args
			// then evaluate the body in the
			// new environment:			
			
			// Evaluate the arguments in the current context
			let values_from_args = self.eval_each(args)?;
			
			// Make a new environment with the current one as the parent
			let mut local_env = self.make_child();
					
			// Add all evaluated args to the child env with the 'params' names
			// according to order in the function call:
			local_env.define_all(*params,values_from_args);
			if TRACE {println!("Created child env\n {}",&local_env.print());}
			local_env.evaluate(*body)			
		}else{
			Err(format!("Can't evaluate as function: {}",&func.print()))
		}

	}
	

	// Assuming it is not a null list and we have an operator or function, pass its cdr in and apply it:
	pub fn apply_operator(&mut self, func:NumericOperator, list:List)-> Result<SExpression, String>{		
		// The cdr (now list) must have at least two items
		if list.is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");			
		}
		
		if list.rest().is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");
					
		}
		
		use crate::primitives::NumericOperator::*;
		let not_implemented = format!("Operator '{}' not implemented",func.print());
		match func {
			Add=> self.add(list),				
			Subtract=> self.subtract(list), 
			Multiply=>  self.multiply(list),
			Divide=>  self.divide(list),
			_=>Err(not_implemented),
		}
	}
	

	// Assuming it is not a null list and we have an operator or function, pass its cdr in and apply it:
	pub fn apply_logical_operator(&mut self, func:LogicalOperator, list:List)-> Result<SExpression, String>{		
		// The cdr (now list) must have at least two items
		if list.is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");			
		}
		
		if list.rest().is_empty(){
			return Err(String::from("Operator ") + func.print() + " requires two arguments");
					
		}		
		use crate::primitives::LogicalOperator::*;
		let not_implemented = format!("Operator '{}' not implemented",func.print());
		match func {
			Greater=> self.eval_greater(list),
			Less=> self.eval_less(list),
			Equal=>self.eval_equal(list),
			Or=> self.eval_or(list),				
			And=>self.eval_and(list), 
			//Not=> eval_not(list),
			//Xor=> eval_xor(list),
			Greater=> Err(not_implemented),
			_ =>Err(not_implemented),
		}
	}
	} // Environment


