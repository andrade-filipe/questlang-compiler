use std::collections::HashMap;
use crate::symbol_table::symbol_type::SymbolType;
use crate::symbol_table::symbol::Symbol;

pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
    scope_level: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            scope_level: 0,
        }
    }

    pub fn insert(&mut self, name: &str, symbol_type: SymbolType, line: usize, column: usize) {
        if self.symbols.contains_key(name) {
            println!(
                "Aviso: A variável '{}' já foi declarada no escopo atual. (Linha {}, Coluna {})",
                name, line, column
            );
        } else {
            self.symbols.insert(
                name.to_string(),
                Symbol {
                    name: name.to_string(),
                    symbol_type,
                    scope_level: self.scope_level,
                    line,
                    column,
                },
            );
        }
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    pub fn enter_scope(&mut self) {
        self.scope_level += 1;
    }

    pub fn exit_scope(&mut self) {
        self.symbols.retain(|_, sym| sym.scope_level < self.scope_level);
        self.scope_level -= 1;
    }

    pub fn print_table(&self) {
        println!("Tabela de Símbolos:");
        for (_, symbol) in &self.symbols {
            println!(
                "  - {} : {:?} (Escopo: {}, Linha: {}, Coluna: {})",
                symbol.name, symbol.symbol_type, symbol.scope_level, symbol.line, symbol.column
            );
        }
    }
}
