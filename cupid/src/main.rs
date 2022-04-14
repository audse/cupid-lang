use cupid::*;


fn main() {
    let mut file_handler = FileHandler::new("src/tests/main.cupid");
    file_handler.run();
    // test_generator();
}