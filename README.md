## Overview

This is a small project to build a Scheme interpreter as a way to learn Rust.

To start with, I was interested in using enum types to model the "atoms" or "cells" in Scheme: A cell can be any primitive type. S-expressions could be stored as enums as well: An s-expression is either a cell or a list. 

Next, I wanted to use real linked lists to store the s-expressions and build the list handling in Rust without resorting to linked-list libraries. This is doing it the hard way for sure; I did it just to learn Rust better.

There are some sample working programs in the  'sample_code' directory.

The interpreter now supports:
* Numeric operators '+','-','*','/'
* Comparisons: '<','=','>'
* Logical operators: 'or','and'
* Integer and floating point literals
* Internally, strings are supported but not string literals
* Definition and use of variables 
* Definition and evaluation of functions
* 'set!' for mutating variables when they can't be changed via recursion
* Local scope within functions
* The 'if' conditional special form
* The 'while' iteration special form
* Multiple expression function bodies
* Multiple expression 'if' and 'while' bodies
* Very basic printing to standard output


Notably missing:
* 'cond' special form
* list handling built-in functions like 'list', 'append', 'car', 'cdr' etc.
* 'do' iteration special form, which is much more lisp-like than the currently implemented 'while'
* Tail call optimization: without it can't really use recursion for many iteration alternatives since we get a stack overflow eventually
* String literals parser support
* Standard input handling and file reading and writing
* Many more minor but important typical Scheme built-ins

S-Expression evaluation takes place in an environment (scope.) "define" works for variables and functions; functions create their own environments and parameters and definitions inside the function are assigned to that local environment.

Some code in "main.rs" shows how one could build tests for language constructs. Just use the __cons()__ function and construct lists of cell values. No need for a tokenizer or REPLat this level. You could perhaps use the s-expression internal representation as a target for a parser of a more conventional type of language.

There's only a basic parser and tokenizer now. Some tests in main take text and parse and interpret it. The app will start up in a REPL mode. You can also pass in a file name of a file with saved code.

### In Progress

* Special commands for the REPL (exit), (env)
* Sample test programs 
* Explore tail call optimization

## To Do

* Replace the current primitive lexer / tokenizer with something better
* Better error handling: Right now I just use Result<SExpression,String> to pass back error messages.
* String literals (may require real parser)
* Code comments (may require real parser)
* Support list data types
* Support vectors and maps




