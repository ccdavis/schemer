mod interpreter;
mod list;
mod primitives;
mod symbolic_expression;

use crate::primitives::Cell;
use crate::list::List;
use crate::list::Link;
use crate::list::cons;

use crate::symbolic_expression::SExpression;
use crate::primitives::NumericOperator;

// Construct some basic list types as tests
fn number_list()->List{
	let c = cons(SExpression::Cell(Cell::Int(25)), 
		cons(SExpression::Cell(Cell::Int(5)), Link::Nil));
	List{head:c}
}


fn main() {
	let n = number_list();	
	let m = List::make_from_cells(vec![Cell::Int(5), Cell::Int(7), Cell::Str("abc".to_string())]);
	let p = List::make_from_sexps(vec![SExpression::List(m.clone()),SExpression::List(n.clone())]);
	
	let summed = List{head:cons(SExpression::Cell(Cell::Op(NumericOperator::Add)), number_list().head)};

	println!("Content: {}", &n.print());	
	println!("Variable type list: {}", &m.print());
	println!("List of lists {}",p.print());
	println!("sum of numbers: {}",summed.evaluate().unwrap().print());
	
	
	
    
	
	
}

