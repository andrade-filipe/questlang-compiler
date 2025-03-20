mod error_handler;
mod lexer;
mod parser;
mod symbol_table;

use lexer::lexer::Lexer;
use parser::parser::Parser;

fn main() {
     // Código de exemplo da linguagem QuestLang, sem semicolons após comandos simples.
     let source_code = r#"
     move_up
     attack
     if (hero) { move_left } else { move_right }
     while (enemy) { jump }
     for (hero; enemy; treasure) { defend }
 "#;

 // Executa o Lexer para obter os tokens
 let mut lexer = Lexer::new(source_code);
 let tokens = lexer.tokenize();

 println!("--- Tokens ---");
 for (token, text) in &tokens {
     println!("{:?} -> '{}'", token, text);
 }
 
 // Executa o Parser com os tokens obtidos
 let mut parser = Parser::new(tokens);
 let ast = parser.parse();
 
 println!("\n--- AST ---");
 for stmt in &ast {
     println!("{:#?}", stmt);
 }
 
 // Reporta erros, se houver
 let errors = parser.into_errors();
 if errors.has_errors() {
     println!("\n--- Erros ---");
     errors.report();
 } else {
     println!("\nSem erros sintáticos.");
 }
}
