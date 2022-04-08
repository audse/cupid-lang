// pub struct Lexer {
//     cursor: usize,
//     chars: Vec<char>,
//     line: usize,
// }

// impl Lexer {
//     pub fn new(string: &str) -> Self {
//         Self {
//             cursor: 0,
//             chars: string.chars().collect(),
//             line: 0,
//         }
//     }

//     pub fn cursor(&self) -> usize {
//         return self.cursor;
//     }

//     pub fn line(&self) -> usize {
//         return self.line;
//     }

//     pub fn peek(&self) -> Option<&char> {
//         return self.chars.get(self.cursor);
//     }

//     pub fn is_done(&self) -> bool {
//         return self.cursor == self.chars.len();
//     }

//     pub fn pop(&mut self) -> Option<&char> {
//         match self.chars.get(self.cursor) {
//             Some(char) => {
//                 self.cursor += 1;
//                 Some(char)
//             },
//             None => None
//         }
//     }
// }

// pub fn string(string: &str) -> bool {
//     let mut lexer = Lexer::new(string);

//     loop {
//         if !unit(&mut lexer) { break; }
//     }

//     return lexer.cursor() > 0;
// }

// pub fn unit(lexer: &mut Lexer) -> bool {
//     match lexer.peek() {
//         Some(char) => if char 
//     }
// }