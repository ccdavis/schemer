## Overview

This is a small project to build a Scheme interpreter as a way to learn Rust.

To start with, I was interested in using enum types to model the "atoms" or "cells" in Scheme: A cell can be any primitive type. S-expressions could be stored as enums as well: An s-expression is either a cell or a list. 

Next, I wanted to use real linked lists to store the s-expressions and build the list handling in Rust.

What exists so far is a core interpreter that can evaluate s-expressions and apply numeric operators. Essentially it is only a calculator for addition at the moment.

The code in "main.rs" shows how one could build tests for language constructs. Just use the __cons()__ function and construct lists of cell values. No need for a tokenizer or REPL at this stage.

## To Do

* Add a tokenizer / lexer to construct s-expressions from text
* Add a REPL
* Better error handling: Right now I just use Result<SExpression,&str> to pass back error messages.
* More language features: Environments, lambdas and function definitions



