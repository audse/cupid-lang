use cupid::test_generator;
    
fn run_file(path: &str) -> std::io::Result<String> {
    let contents = std::fs::read_to_string(path)?;
    return Ok(contents);
}


fn main() {
    // let contents = run_file("/Volumes/Macintosh HD/Users/audreyserene/Projects/cupid-lang/cupid/src/tests/main.cupid").expect("bad file");
    // 
    // let mut scope = CupidScope::new(None);
    // 
    // let mut lexer = Lexer::new(contents, true);
    // lexer.scan();
    // 
    // let mut parser = Parser::new(lexer);
    // 
    // let mut result = CupidValue::None;
    // let block = parser.parse();
    // 
    // for exp in block {
    //    result = exp.resolve(&mut scope);
    // }
    
    // println!("{} {}", result, scope)
    test_generator();
}