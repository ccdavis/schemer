use crate::symbolic_expression::SExpression;
use std::collections::HashMap;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1


#[derive(Debug,Clone,Copy,EnumIter)]
pub enum NumericOperator{
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulo,
}

#[derive(Debug,Clone,Copy,EnumIter)]
pub enum LogicalOperator{
	Less,
	Greater,
	Equal,
	GreaterEqual,
	LessEqual,
	Or,
	And,
	Not,
	Xor,	
}

impl NumericOperator{

	// Defining print() to give the lexeme for the token  (Add == '+' for instance): This 
	// is something That would take just as much code implementing the display trait. And,
	// I want to still have the option to use to_string() to give the debug name ('Add' for
	// the Add token for example.)
	//
	// Strum macros offers a way to convert enums to strings, but again I want to have different
	// varient  names in the source from the token representation in the target language.
	pub fn print(self)->&'static str{
		match self{
			NumericOperator::Add=>"+",
			NumericOperator::Subtract=>"-",
			NumericOperator::Multiply=>"*",
			NumericOperator::Divide=>"/",
			NumericOperator::Modulo=>"%",			
		}
	}
}

impl LogicalOperator{
	pub fn print(self)->&'static str{
		match self{
			LogicalOperator::Greater=>">",
			LogicalOperator::Less=>"<",
			LogicalOperator::Equal=>"=",
			LogicalOperator::GreaterEqual=>">=",
			LogicalOperator::LessEqual=>"<=",			
			LogicalOperator::Or=>"or",
			LogicalOperator::And=>"and",
			LogicalOperator::Not=>"not",
			LogicalOperator::Xor=>"xor",
			
		}
	}		
}

#[derive(Debug,Clone,Copy,EnumIter)]
pub enum SpecialForm{
	Define,	
	Let,
	SetCar,
	Cond,
	If,
	Map,
	Filter,
	Count,
	Cons,
	List,
	Car,
	Cdr,
	
	Env,
	Exit,
	Input,
	Output,		
}

impl SpecialForm{
	fn print(self)->&'static str{
		match self{
			SpecialForm::Define=>"define",
			SpecialForm::Let=>"let",
			SpecialForm::SetCar=>"setcar!",
			SpecialForm::Cond=>"cond",
			SpecialForm::If=>"if",
			SpecialForm::Map=>"map",
			SpecialForm::Filter=>"filter",
			SpecialForm::Count=>"count",
			SpecialForm::Cons=>"cons",
			SpecialForm::List=>"list",
			SpecialForm::Car=>"car",
			SpecialForm::Cdr=>"cdr",
			SpecialForm::Env=>"env",
			SpecialForm::Exit=>"exit",
			SpecialForm::Input=>"input",
			SpecialForm::Output=>"output",			
		}
	}
}


#[derive(Clone)]
pub enum Cell{
	Int(i64),
	Flt(f64),
	Str(String),
	Bool(bool),
	Symbol(i32,String),	
	Op(NumericOperator),
	Logical(LogicalOperator),
	Special(SpecialForm),	 // other built-in functions
	Lambda(Box<SExpression>,Box<SExpression>), // arguments and body of the lambda
}

impl Cell{

	// Returns a map for use in parsing
	
	pub fn print(&self)->String{
		match &self{
			Cell::Int(value)=>value.to_string(),
			Cell::Flt(value)=>value.to_string(),
			Cell::Str(value)=>value.to_string(),
			Cell::Symbol(number,name)=>{String::from("Symbol ") + &number.to_string() + &name},
			Cell::Bool(value) => value.to_string(),
			Cell::Op(operator) => String::from(format!("Numeric operator {}", operator.print())),
			Cell::Logical(operator)=>String::from(format!("Logical operator{}", operator.print())),
			Cell::Special(special_form)=> String::from("Special form"),			
			Cell::Lambda(_,_) => String::from("Lambda: ")
		}		
	}
	
	// Convenience for  implementing numeric operators
	pub fn eval_as_number(&self)->Result<Cell, String>{
		match self{
			Cell::Int(value)=>Ok(self.clone()),
			Cell::Flt(value)=>Ok(self.clone()),
			_ => Err("Not a number type!".to_string()),
		}
	}

	// Convenience for  implementing logical operators
	// Numbers can be arguments to logical ops like "or", "and" like (and 2 5 0) which would be false.
	// These are not bit-wise operators.
	pub fn eval_as_bool(&self)->Result<Cell, String>{
		match self{
			Cell::Int(value)=> {
				let gt0:bool = 0 < *value;
				Ok(Cell::Bool(gt0))
			},
			Cell::Flt(value)=>{
				let gt0:bool = *value > 0.0; 
				Ok(Cell::Bool(gt0))
			},
			Cell::Bool(value) => Ok(self.clone()),
			_ => Err("Not a boolean type!".to_string()),
		}
	}
} // impl Cell

	// This is a helper for the parser
	pub fn map_cell_from_string() -> HashMap<String, Cell>{
		let mut tokens:HashMap<String,Cell> = HashMap::new();		
		for numeric_op in NumericOperator::iter(){
			let c = Cell::Op(numeric_op);			
			tokens.insert(String::from(numeric_op.print()), c);			
		}
		
		for boolean_op in LogicalOperator::iter(){
			let c = Cell::Logical(boolean_op);			
			tokens.insert(String::from(boolean_op.print()), c);			
		}
		
		for form in SpecialForm::iter(){
			let c = Cell::Special(form);			
			tokens.insert(String::from(form.print()), c);
		}
			
		tokens
	}
		