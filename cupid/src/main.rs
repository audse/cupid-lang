use cupid::*;
    
fn run_file(path: &str) -> std::io::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    return Ok(contents);
}

pub fn run(path: &str, debug: bool) {
    let contents = std::fs::read_to_string(path).unwrap_or(String::from("false"));
    if debug { println!("Contents: {:?}", contents); }
    
    let mut parser = CupidParser::new(contents);
    let parse_tree = parser._file(None);
    if debug { println!("Parse Tree: {:#?}", parse_tree); }
    
    let semantics = to_tree(&parse_tree.unwrap().0);
    if debug { println!("Semantics: {:#?}", semantics); }
    
    let mut scope = Scope::new(None);
    let result = semantics.resolve(&mut scope);
    println!("Result: {:#?}", result);
}

fn main() {
    run("src/tests/main.cupid", false);
    // test_generator();
}