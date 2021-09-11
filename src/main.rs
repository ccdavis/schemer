mod interpreter;
mod list;
mod primitives;
mod symbolic_expression;
mod parser;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs;

use crate::primitives::Cell;
use crate::list::List;
use crate::list::Link;
use crate::list::cons;

use crate::symbolic_expression::SExpression;
use crate::primitives::NumericOperator;


// Put this in the REPL loop
fn interpret(program:String, environment:&mut interpreter::Environment)->String{
	let tokens = parser::lex(program);
	let p = parser::Parser::new();
	match p.parse(&tokens){
		Ok((valid_ast,_)) =>{		
			// Parser seemed to work, so attempt to interpret the AST
			let r = environment.evaluate(valid_ast);			
			// Check for interpreter errors
			match r {
				Ok(good_result) => good_result.print(),
				Err(error) => format!("Interpreter Error: {}", error),
			}
		},
		// Parsing errors
		Err(error) =>{
			// handle different types of errors
			match error{
				parser::ParseError::Reason(reason) =>format!("{}", reason),
			}
		}
	}
}




fn interpret_top_level(program:String, environment:&mut interpreter::Environment)->String{
	let mut main_program:String  = String::from("(") + &program + ")";
	let tokens = parser::lex(main_program);
	let p = parser::Parser::new();
	
	match p.parse(&tokens){
		Ok((valid_ast,_)) =>		
			// Parser seemed to work, so attempt to interpret the AST
			match valid_ast{
				SExpression::List(list)=>
					match environment.eval_each(list){
						Ok(items)=>items.iter()
							.map(|i| i.print())
							.collect::<Vec<String>>()
							.join("\n"),
						Err(error)=>format!("Interpreter error {}",error),
					},
				_=>
					match environment.evaluate(valid_ast){
						Ok(ref result)=>result.print(),
						Err(error)=>format!("Interpreter error {}",error),
					},
			},			
		Err(error) =>{
			// handle different types of errors
			match error{
				parser::ParseError::Reason(reason) =>format!("{}", reason),
			}
		}
	}
}




// From the Rustyline README
fn repl(){	
    // `()` can be used when no completer is required
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
	
	// The environment for the duration of the REPL session
	let mut envr = interpreter::Environment::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());								
				let results = interpret(line, &mut envr);
                println!("=>  {}", &results);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();	
}

// Construct some basic list types as tests
fn number_list()->List{
	let c = cons(SExpression::Cell(Cell::Int(25)), 
		cons(SExpression::Cell(Cell::Int(5)), Link::Nil));
	List{head:c}
}

// These aren't unit or acceptance  tests, but a place to put experimental
// language features and see what crashes.
fn run_tests(){
	let n = number_list();	
	let m = List::make_from_cells(vec![Cell::Int(5), Cell::Int(7), Cell::Str("abc".to_string())]);
	let p = List::make_from_sexps(vec![SExpression::List(m.clone()),SExpression::List(n.clone())]);
	
	let summed = List{head:cons(SExpression::Cell(Cell::Op(NumericOperator::Add)), number_list().head)};

	println!("Content: {}", &n.print());	
	println!("Variable type list: {}", &m.print());
	println!("List of lists {}",p.print());
	
	
	let mut envr = interpreter::Environment::new();
	println!("sum of numbers: {}",summed.evaluate(&mut envr).unwrap().print());
	
	
	
	println!("{}",
		interpret(String::from("(+ 1 2 3 (+ 5 6))"),&mut envr));
		
	println!("{}",interpret(String::from("(+ 1 2 3 (* 5 6))"),&mut envr));
	
	println!("{}",interpret(String::from("(/ 10 5)"),&mut envr));
	println!("{}",interpret(String::from("(/ 5 10)"),&mut envr));
	println!("{}",interpret(String::from("(* 8 (/ 5 10))"),&mut envr));
	println!("{}",interpret(String::from("(- 8 (* 2 25) (+ 2 3) (/ 5 10))"),&mut envr));
}

fn main() {
	let args = std::env::args().collect::<Vec<String>>();
	
	if args.len()<2{
		repl();
	}else{
		let program_file = &args[1];
		let code = fs::read_to_string(program_file)
			.expect(&format!("File at {} unreadable.",program_file));
			
		let mut envr = interpreter::Environment::new();
		let results = interpret_top_level(code, &mut envr);
		println!("=>  {}", &results);
		
		
		
	}
	
}
	