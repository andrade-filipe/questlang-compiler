use crate::symbol_table::symbol_type::SymbolType;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub scope_level: usize,
    pub line: usize,
    pub column: usize,
}