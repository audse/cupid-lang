use cupid::*;


fn main() {
    let mut file_handler = FileHandler::new("src/tests/library.cupid");
    file_handler.run();
    // test_generator();
}