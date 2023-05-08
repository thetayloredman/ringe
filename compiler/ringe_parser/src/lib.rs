//! Lexing and parsing utilities for the Ringe C compiler

use lalrpop_util::lalrpop_mod;

lalrpop_util::lalrpop_mod!(internal_parser);

pub mod lexer;
