
struct Node {
	name: String,
	children: Vec<Node>,
}
struct Parser;
impl Parser {
	
	fn operation(&mut self) -> Option<Node> {
		let pos = self.mark();
		
		if let (Some(term), Some(str1), Some(term2)) = (self.term(), self.expect('+'), self.term()) { 
			return Node(operation, [term, '+', term]); 
		}
		self.reset(pos);
	
		return None;
	}
	
	
	fn term(&mut self) -> Option<Node> {
		let pos = self.mark();
		
		if let (Some(name)) = (self.expect_name()) { 
			return Node(term, [name]); 
		}
		self.reset(pos);
	
	
		if let (Some(number)) = (self.expect_number()) { 
			return Node(term, [number]); 
		}
		self.reset(pos);
	
		return None;
	}

}
