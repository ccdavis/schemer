use crate::symbolic_expression::SExpression;
use std::collections::HashMap;

use strum::IntoEnumIterator; // 0.17.1
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum CoreFunc {
    // A subset of the "essential procedures" pertaining to type checking and conversion
    IsChar,
    IsNumber,
    IsList,
    IsNull,
    IsBoolean,
    IsExact,
    IsString,
    NumberToString,
    StringToNumber,
    SymbolToString,
    StringToSymbol,
    CharToNumber,
    NumberToChar,

    // Essential functions for lists
    Map,
    Filter,
    Count,
    Cons,
    List,
    Car,
    Cdr,
    First,
    Rest,
    Append,
}

impl CoreFunc {
    pub fn print(self) -> &'static str {
        match self {
            CoreFunc::IsChar => "char?",
            CoreFunc::IsNumber => "number?",
            CoreFunc::IsList => "list?",
            CoreFunc::IsNull => "null?",
            CoreFunc::IsBoolean => "boolean?",
            CoreFunc::IsString => "string?",
            CoreFunc::IsExact => "exact?", // floats are inexact
            CoreFunc::NumberToString => "number->string",
            CoreFunc::StringToNumber => "string->number",
            CoreFunc::SymbolToString => "symbol->string",
            CoreFunc::StringToSymbol => "string->symbol",
            CoreFunc::CharToNumber => "char->number",
            CoreFunc::NumberToChar => "number->char",
            CoreFunc::Map => "map",
            CoreFunc::Filter => "filter",
            CoreFunc::Count => "count",
            CoreFunc::Cons => "cons",
            CoreFunc::List => "list",
            CoreFunc::Car => "car",
            CoreFunc::Cdr => "cdr",
            CoreFunc::First => "first", // aliases for car and cdr
            CoreFunc::Rest => "rest",
            CoreFunc::Append => "append",
        }
    }
} // impl corefunc
#[derive(Debug, Clone, Copy, EnumIter)]
pub enum NumericOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    /*
    Other Scheme built-in numeric procedures https://www.cs.cmu.edu/Groups/AI/html/r4rs/r4rs_8.html
    Abs,
    Min,
    Max,
    Quotient,
    Remainder,
    Numerator,
    Denominator,
    Gcd,
    Lcm,
    Floor,
    Ceiling,
    Truncate,
    Round,
    Rationalize,
    Expt,

    */
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum LogicalOperator {
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

impl NumericOperator {
    // Defining print() to give the lexeme for the token  (Add == '+' for instance): This
    // is something That would take just as much code implementing the display trait. And,
    // I want to still have the option to use to_string() to give the debug name ('Add' for
    // the Add token for example.)
    //
    // Strum macros offers a way to convert enums to strings, but again I want to have different
    // varient  names in the source from the token representation in the target language.
    pub fn print(self) -> &'static str {
        match self {
            NumericOperator::Add => "+",
            NumericOperator::Subtract => "-",
            NumericOperator::Multiply => "*",
            NumericOperator::Divide => "/",
            NumericOperator::Modulo => "%",
        }
    }
}

impl LogicalOperator {
    pub fn print(self) -> &'static str {
        match self {
            LogicalOperator::Greater => ">",
            LogicalOperator::Less => "<",
            LogicalOperator::Equal => "=",
            LogicalOperator::GreaterEqual => ">=",
            LogicalOperator::LessEqual => "<=",
            LogicalOperator::Or => "or",
            LogicalOperator::And => "and",
            LogicalOperator::Not => "not",
            LogicalOperator::Xor => "xor",
        }
    }
}

#[derive(Debug, Clone, Copy, EnumIter)]
pub enum SpecialForm {
    Define,
    Let,
    SetCar,
    Set,
    Cond,
    If,
    Do,
    While,
    When,
    Unless,
    Break,
    Continue,
    Begin,

    Env,
    Exit,
    Input,
    Output,
    OutputLine,
}

impl SpecialForm {
    pub fn print(self) -> &'static str {
        match self {
            SpecialForm::Define => "define",
            SpecialForm::Let => "let",
            SpecialForm::SetCar => "setcar!",
            SpecialForm::Set => "set!",
            SpecialForm::Cond => "cond",
            SpecialForm::If => "if",
            SpecialForm::Do => "do",
            SpecialForm::While => "while",
            SpecialForm::When => "when",
            SpecialForm::Unless => "unless",
            SpecialForm::Break => "break",
            SpecialForm::Continue => "continue",
            SpecialForm::Begin => "begin",
            SpecialForm::Env => "env",
            SpecialForm::Exit => "exit",
            SpecialForm::Input => "input",
            SpecialForm::Output => "output",
            SpecialForm::OutputLine => "output-line",
        }
    }
}

#[derive(Clone)]
pub enum Cell {
    Int(i64),
    Flt(f64),
    Str(String),
    Bool(bool),
    Symbol(i32, String),
    Op(NumericOperator),
    Logical(LogicalOperator),
    Special(SpecialForm), // other built-in functions
    Core(CoreFunc),
    Lambda(Box<SExpression>, Box<SExpression>), // arguments and body of the lambda
}

impl Cell {
    pub fn print(&self) -> String {
        match &self {
            Cell::Int(value) => value.to_string(),
            Cell::Flt(value) => value.to_string(),
            Cell::Str(value) => value.to_string(),
            Cell::Symbol(number, name) => format!("Symbol {}: {}", number, &name),
            Cell::Bool(value) => value.to_string(),
            Cell::Op(operator) => format!("Numeric operator {}", operator.print()),
            Cell::Logical(operator) => format!("Logical operator{}", operator.print()),
            Cell::Special(special_form) => String::from("Special form"),
            Cell::Core(func) => String::from("core-function"),
            Cell::Lambda(_, _) => String::from("Lambda: "),
        }
    }

    // Convenience for  implementing numeric operators
    pub fn eval_as_number(&self) -> Result<Cell, String> {
        match self {
            Cell::Int(_) => Ok(self.clone()),
            Cell::Flt(_) => Ok(self.clone()),
            _ => Err("Not a number type!".to_string()),
        }
    }

    // Convenience for  implementing logical operators
    // Numbers can be arguments to logical ops like "or", "and" like (and 2 5 0) which would be false.
    // These are not bit-wise operators.
    pub fn eval_as_bool(&self) -> Result<Cell, String> {
        match self {
            Cell::Int(value) => {
                let gt0: bool = 0 < *value;
                Ok(Cell::Bool(gt0))
            }
            Cell::Flt(value) => {
                let gt0: bool = *value > 0.0;
                Ok(Cell::Bool(gt0))
            }
            Cell::Bool(_) => Ok(self.clone()),
            _ => Err("Not a boolean type!".to_string()),
        }
    }
} // impl Cell

// This is a helper for the parser
pub fn map_cell_from_string() -> HashMap<String, Cell> {
    let mut tokens: HashMap<String, Cell> = HashMap::new();

    for func in CoreFunc::iter() {
        let c = Cell::Core(func);
        tokens.insert(String::from(func.print()), c);
    }

    for numeric_op in NumericOperator::iter() {
        let c = Cell::Op(numeric_op);
        tokens.insert(String::from(numeric_op.print()), c);
    }

    for boolean_op in LogicalOperator::iter() {
        let c = Cell::Logical(boolean_op);
        tokens.insert(String::from(boolean_op.print()), c);
    }

    for form in SpecialForm::iter() {
        let c = Cell::Special(form);
        tokens.insert(String::from(form.print()), c);
    }

    tokens
}
