use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Integer,
    Boolean,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope_level: usize,
    pub line: usize,
    pub column: usize,
}

/// Tabela de símbolos com suporte a escopos
pub struct SymbolTable {
    symbols: HashMap<String, Symbol>,
    scope_level: usize, // Nível atual do escopo
}

impl SymbolTable {
    /// Cria uma nova tabela de símbolos
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            scope_level: 0,
        }
    }

    /// Insere um novo símbolo na tabela
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

    /// Consulta um símbolo na tabela
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }

    /// Entra em um novo escopo
    pub fn enter_scope(&mut self) {
        self.scope_level += 1;
    }

    /// Sai do escopo atual e remove símbolos do escopo anterior
    pub fn exit_scope(&mut self) {
        self.symbols.retain(|_, sym| sym.scope_level < self.scope_level);
        self.scope_level -= 1;
    }

    /// Exibe todos os símbolos armazenados (para debug)
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
