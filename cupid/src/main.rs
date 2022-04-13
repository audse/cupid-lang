use cupid::*;


fn main() {
    let mut file_handler = FileHandler::new("src/tests/main.cupid");
    file_handler.run_debug();
    // test_generator();
}