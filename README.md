## Overview

This is a small project to build a Scheme interpreter as a way to learn Rust.

To start with, I was interested in using enum types to model the "atoms" or "cells" in Scheme: A cell can be any primitive type. S-expressions could be stored as enums as well: An s-expression is either a cell or a list. 

Next, I wanted to use real linked lists to store the s-expressions and build the list handling in Rust without resorting to linked-list libraries.

What exists so far is a core interpreter that can evaluate s-expressions and apply numeric operators, define variables and define functions.  The interpreter has all basic arithmetic operations and also one comparison operator '>' and one logical operator 'or', on the way to implementing all basic comparison and logical operators. The only conditional expression is 'if' so far.

S-Expression evaluation takes place in an environment. "define" works for variables and functions; functions create their own environments and parameters and definitions inside the function are assigned to that local environment.

Some code in "main.rs" shows how one could build tests for language constructs. Just use the __cons()__ function and construct lists of cell values. No need for a tokenizer or REPLat this level. You could perhaps use the s-expression internal representation as a target for a parser of a more conventional type of language.

There's only a basic parser and tokenizer now. Some tests in main take text and parse and interpret it. The app will start up in a REPL mode. You can also pass in a file name of a file with saved code.

### In Progress

* Special commands for the REPL (exit), (env)
* Sample test programs 
* Explore tail call optimization
* Support at least one iterative special form like "loop" or "do" or "while"

## To Do

* Replace the current primitive lexer / tokenizer with something better
* Better error handling: Right now I just use Result<SExpression,String> to pass back error messages.




