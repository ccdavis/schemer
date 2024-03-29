use crate::interpreter::Environment;
use crate::primitives::Cell;
use crate::symbolic_expression::SExpression;

#[derive(Clone)]
pub struct Pair {
    car: Box<SExpression>,
    cdr: Link,
}

#[derive(Clone)]
pub enum Link {
    Data(Box<Pair>),
    Nil,
}

pub fn cons(object: SExpression, list: Link) -> Link {
    Link::Data(Box::<Pair>::new(Pair {
        car: Box::<SExpression>::new(object),
        cdr: list,
    }))
}

#[derive(Clone)]
pub struct List {
    pub head: Link,
}

impl List {
    pub fn is_empty(&self) -> bool {
        match &self.head {
            Link::Nil => true,
            Link::Data(_) => false,
        }
    }

    pub fn first(&self) -> Box<SExpression> {
        match &self.head {
            Link::Nil => {
                // runtime error?
                Box::<SExpression>::new(SExpression::Null)
            }
            Link::Data(ref pair) => pair.car.clone(),
        }
    }

    pub fn rest(&self) -> List {
        match &self.head {
            Link::Nil => {
                // runtime error?
                List { head: Link::Nil }
            }
            Link::Data(ref pair) => List {
                head: pair.cdr.clone(),
            },
        }
    }

    // Convenience for making lists of primitives
    pub fn make_from_cells(objects: Vec<Cell>) -> List {
        let tail: Link = Link::Nil;
        let mut head: Link = tail;
        for index in (0..objects.len()).rev() {
            head = cons(SExpression::Cell(objects[index].clone()), head);
        }
        List { head: head }
    }

    // If the second argument is a list wemove the head of the list to the left
    // hand argument rather than making a (list exp (list exp2))
    // 'cons' in scheme  works like:
    //
    // > (cons 1 (list 2 3))
    // -> > (1 2 3)
    pub fn construct_list(exp1: SExpression, exp2: SExpression) -> List {
        let new_link = match exp2 {
            SExpression::List(list) => cons(exp1, list.head),
            _ => {
                let end_link = cons(exp2, Link::Nil);
                cons(exp1, end_link)
            }
        };
        List { head: new_link }
    }

    pub fn make_from_sexps(exps: Vec<SExpression>) -> List {
        let tail: Link = Link::Nil;
        let mut head: Link = tail;
        for index in (0..exps.len()).rev() {
            head = cons(exps[index].clone(), head);
        }
        List { head: head }
    }

    pub fn print(&self) -> String {
        match &self.head {
            Link::Nil => String::from("()"),
            Link::Data(_) => {
                let first_print: String = self.first().print();
                first_print + " " + &self.rest().print()
            }
        }
    }

    pub fn evaluate(&self, envr: &mut Environment) -> Result<SExpression, String> {
        let car = self.first();

        // A list with a first cell of an operator or user-defined function
        // must apply that function / operator to the rest of the list.
        match *car {
            SExpression::Cell(cell) => match cell {
                Cell::Special(form) => envr.apply_special_form(form, self.rest()),
                Cell::Core(func) => envr.apply_core_func(func, self.rest()),
                // If it's a symbol at the head of the list, it must be a function call
                Cell::Symbol(number, name) => envr.apply_function(number, name, self.rest()),
                Cell::Op(operator) => envr.apply_operator(operator, self.rest()),
                Cell::Logical(operator) => envr.apply_logical_operator(operator, self.rest()),
                _ => Err("Evaluation on this cell type  not supported".to_string()),
            },
            SExpression::List(sub_list) => sub_list.evaluate(envr),
            SExpression::Null => Ok(SExpression::Null),
        }
    }
} // list impl
