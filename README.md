## Overview

This is a small project to build a Scheme interpreter as a way to learn Rust.

To start with, I was interested in using enum types to model the "atoms" or "cells" in Scheme: A cell can be any primitive type. S-expressions could be stored as enums as well: An s-expression is either a cell or a list. 

Next, I wanted to use real linked lists to store the s-expressions and build the list handling in Rust without resorting to linked-list libraries.

What exists so far is a core interpreter that can evaluate s-expressions and apply numeric operators. The interpreter has all basic arithmetic operations and also one comparison operator '>' and one logical operator 'or', on the way to implementing all basic comparison and logical operators.

Some code in "main.rs" shows how one could build tests for language constructs. Just use the __cons()__ function and construct lists of cell values. No need for a tokenizer or REPLat this level.

There's a basic parser and tokenizer now. Some tests in main take text and parse and interpret it.

The s-expression evaluation takes place in an environment but without implementing 'define' or 'let' it doesn't change any behavior.

### In Progress

* Lambdas
* Make the environment scoped
* Special commands for the REPL (exit), (env)

## To Do

* Debug 'define'
* Implement lambdas
* Replace the current primitive lexer / tokenizer with something better
* Better error handling: Right now I just use Result<SExpression,&str> to pass back error messages.




