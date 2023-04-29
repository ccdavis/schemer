use crate::list::List;
use crate::primitives::Cell;

#[derive(Clone)]
pub enum SExpression {
    List(List),
    Cell(Cell),
    Null,
}

impl SExpression {
    pub fn print(&self) -> String {
        match &self {
            &SExpression::Cell(cell) => cell.print(),
            &SExpression::List(list) => list.print(),
            SExpression::Null => String::from("Null"),
        }
    }

    pub fn as_number(self) -> Result<Cell, String> {
        match self {
            SExpression::Cell(cell) => cell.eval_as_number(),
            _ => Err(String::from("Not a number type")),
        }
    }

    pub fn as_bool(self) -> Result<Cell, String> {
        match self {
            SExpression::Cell(cell) => cell.eval_as_bool(),
            _ => Err(String::from("Not a boolean type")),
        }
    }

    pub fn as_rust_bool(self) -> Result<bool, String> {
        let bool_cell = self.as_bool()?;
        match bool_cell {
            Cell::Bool(truth) => Ok(truth),
            _ => Err(format!("Not a boolean type {}", bool_cell.print())),
        }
    }
} // impl SExpression
