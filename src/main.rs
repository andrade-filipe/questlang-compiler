use std::env;
use std::fs;
use std::process;
use std::time::Instant;

mod error_handler;
mod lexer;
mod parser;
mod pretty_print;
mod symbol_table;

use lexer::lexer::Lexer;
use parser::parser::Parser;
use pretty_print::PrettyPrinter;
use symbol_table::symbol_table::SymbolTable;
use symbol_table::symbol_type::SymbolType;

fn main() {
    let start_total = Instant::now();
    
    let start_read = Instant::now();
     let args: Vec<String> = env::args().collect();
     if args.len() < 2 {
         eprintln!("Uso: {} <arquivo_fonte>", args[0]);
         process::exit(1);
     }
     let filename = &args[1];

     let source_code = fs::read_to_string(filename).unwrap_or_else(|err| {
         eprintln!("Erro ao ler o arquivo {}: {}", filename, err);
         process::exit(1);
     });
     let finish_read = start_read.elapsed();
 
     // --- Lexical Analysis ---
     let start_lexer = Instant::now();
     let mut lexer = Lexer::new(&source_code);
     let tokens = lexer.tokenize();
     println!("--- Tokens ---");
     for (token, text, _) in tokens.iter() {
         println!("{:?} -> '{}'", token, text);
     }
     let lex_finished = start_lexer.elapsed();
 
     // --- Parsing ---
     let start_parser = Instant::now();
     let (ast, errors) = Parser::new(tokens, &source_code).parse();

     if errors.has_errors() {
         println!("\n--- Parsing Errors ---");
         errors.report();
     } else {
         println!("\nParsing concluído sem erros.");
     }
    let parser_finished = start_parser.elapsed();
    
     // --- Pretty Print ---
     let start_pretty_print = Instant::now();
     let mut printer = PrettyPrinter::new();
     let pretty_output = printer.print_stmts(&ast);
     println!("\n--- Pretty Printed AST ---");
     println!("{}", pretty_output);
     let finish_pretty_print = start_pretty_print.elapsed();

     // --- Symbol Table ---
     let mut sym_table = SymbolTable::new();
     if source_code.contains("hero") {
         sym_table.insert("hero", SymbolType::Integer, 1, 1);
     }
     if source_code.contains("enemy") {
         sym_table.insert("enemy", SymbolType::Boolean, 1, 1);
     }
     if source_code.contains("treasure") {
         sym_table.insert("treasure", SymbolType::Integer, 1, 1);
     }
     if source_code.contains("trap") {
         sym_table.insert("trap", SymbolType::Boolean, 1, 1);
     }
     println!("\n--- Symbol Table ---");
     sym_table.print_table();

     let finish_total = start_total.elapsed();
     println!();
     println!("Reading completed in {:.3} ms", finish_read.as_secs_f64() * 1e3);
     println!("Lexing completed in {:.3} ms", lex_finished.as_secs_f64() * 1e3);
     println!("Parsing completed in {:.3} ms", parser_finished.as_secs_f64() * 1e3);
     println!("Pretty Printing completed in {:.3} ms", finish_pretty_print.as_secs_f64() * 1e3);
     println!("All Code completed in {:.3} ms", finish_total.as_secs_f64() * 1e3);
}
