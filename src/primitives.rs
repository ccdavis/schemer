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
pub enum BooleanOperator{
	Less,
	Greater,
	Equal,
	NotEqual,
}

impl NumericOperator{
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

impl BooleanOperator{
	pub fn print(self)->&'static str{
		match self{
			BooleanOperator::Greater=>">",
			BooleanOperator::Less=>"<",
			BooleanOperator::Equal=>"=",
			BooleanOperator::NotEqual=>"<>",			
		}
	}		
}

#[derive(Debug,Clone,Copy,EnumIter)]
pub enum SpecialForm{
	DefineFunction,
	DefineVariable,
	LetVariable,
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
}


#[derive(Clone)]
pub enum Cell{
	Int(i64),
	Flt(f64),
	Str(String),
	Bool(bool),
	Symbol(i32,String),	
	Op(NumericOperator),
	Compare(BooleanOperator),
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
			Cell::Op(operator) => String::from("Numeric operator"),
			Cell::Compare(operator)=>String::from("comparison operator"),
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


} // impl Cell


	pub fn map_cell_from_string() -> HashMap<String, Cell>{
		let mut cells:HashMap<String,Cell> = HashMap::new();		
		for numeric_op in NumericOperator::iter(){
			let c = Cell::Op(numeric_op);			
			cells.insert(String::from(numeric_op.print()), c);			
		}
			
		cells
	}
		