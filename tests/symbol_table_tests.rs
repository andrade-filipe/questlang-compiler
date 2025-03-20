use questlang::symbol_table::symbol_table::SymbolTable;
use questlang::symbol_table::symbol_type::SymbolType;

#[test]
fn test_insertion_and_lookup() {
    let mut table = SymbolTable::new();

    // Insere símbolos no escopo global
    table.insert("hero", SymbolType::Integer, 1, 1);
    table.insert("enemy", SymbolType::Boolean, 2, 2);

    // Verifica se os símbolos existem
    let hero = table.lookup("hero");
    assert!(hero.is_some(), "Símbolo 'hero' deve existir");
    let enemy = table.lookup("enemy");
    assert!(enemy.is_some(), "Símbolo 'enemy' deve existir");

    // Verifica que um símbolo não inserido retorna None
    assert!(table.lookup("treasure").is_none(), "Símbolo 'treasure' não deve existir");
}

#[test]
fn test_duplicate_insertion() {
    let mut table = SymbolTable::new();

    // Insere 'hero' no escopo global
    table.insert("hero", SymbolType::Integer, 1, 1);
    // Tenta inserir novamente 'hero'; espera-se que não sobrescreva o primeiro
    table.insert("hero", SymbolType::Integer, 2, 2);
    
    let hero = table.lookup("hero").unwrap();
    // Os metadados devem permanecer os da primeira inserção
    assert_eq!(hero.line, 1, "A primeira linha de inserção deve ser mantida");
    assert_eq!(hero.column, 1, "A primeira coluna de inserção deve ser mantida");
}

#[test]
fn test_scope_handling() {
    let mut table = SymbolTable::new();

    // Insere símbolos no escopo global
    table.insert("hero", SymbolType::Integer, 1, 1);
    table.insert("enemy", SymbolType::Boolean, 2, 2);
    
    // Entra em um novo escopo e insere um símbolo
    table.enter_scope();
    table.insert("treasure", SymbolType::Integer, 3, 3);

    // No escopo atual, todos devem estar visíveis
    assert!(table.lookup("hero").is_some(), "Símbolo 'hero' deve persistir no escopo global");
    assert!(table.lookup("enemy").is_some(), "Símbolo 'enemy' deve persistir no escopo global");
    assert!(table.lookup("treasure").is_some(), "Símbolo 'treasure' deve existir no escopo interno");

    // Sai do escopo; 'treasure' deve ser removido
    table.exit_scope();
    assert!(table.lookup("treasure").is_none(), "Símbolo 'treasure' deve ser removido ao sair do escopo");
    assert!(table.lookup("hero").is_some(), "Símbolo 'hero' deve continuar existindo");
    assert!(table.lookup("enemy").is_some(), "Símbolo 'enemy' deve continuar existindo");
}

#[test]
fn test_nested_scopes() {
    let mut table = SymbolTable::new();

    // Insere um símbolo global
    table.insert("global", SymbolType::Integer, 1, 1);
    
    // Entra em um primeiro escopo
    table.enter_scope();
    table.insert("level1", SymbolType::Boolean, 2, 2);
    
    // Entra em um escopo aninhado
    table.enter_scope();
    table.insert("level2", SymbolType::Integer, 3, 3);
    
    // Todos os símbolos devem estar visíveis no escopo mais interno
    assert!(table.lookup("global").is_some());
    assert!(table.lookup("level1").is_some());
    assert!(table.lookup("level2").is_some());
    
    // Sai do escopo mais interno; 'level2' deve desaparecer
    table.exit_scope();
    assert!(table.lookup("level2").is_none());
    assert!(table.lookup("level1").is_some());
    
    // Sai do primeiro escopo; 'level1' deve desaparecer
    table.exit_scope();
    assert!(table.lookup("level1").is_none());
    assert!(table.lookup("global").is_some());
}

#[test]
fn test_scope_decrease_does_not_remove_global_symbols() {
    let mut table = SymbolTable::new();
    table.insert("hero", SymbolType::Integer, 1, 1);
    
    // Entra e sai de um escopo sem adicionar símbolos
    table.enter_scope();
    table.exit_scope();
    
    // O símbolo global deve continuar existindo
    assert!(table.lookup("hero").is_some());
}

/// Teste opcional: captura da saída do print_table para garantir que não haja panics.
/// Para esse teste, apenas chamamos print_table e verificamos que não há panics.
#[test]
fn test_print_table_output() {
    let mut table = SymbolTable::new();
    table.insert("hero", SymbolType::Integer, 1, 1);
    table.insert("enemy", SymbolType::Boolean, 2, 2);
    table.print_table();
}
