const cupidGrammar = ohm.grammar(String.raw`
CupidLang {   


	Block
    = Line
    | Function
       
    
    // FEATURES
    
    Function
    = Type Identifier ":" ( Type Identifier ","* )* FunctionBlock
    
    FunctionBlock
    = "{"
    	(~"}" (Line|Function) )*
      "}" ~"\n"
    
    Line
    = ( Assignment | Type | Write ) ~"\n"
    
    Assignment
    = Type Identifier AnyType ~"\n"
    
	Type
    = "(" ( Keyword | Identifier ) ")"
    
    Write
    = write AnyType ~"\n"
    
    
    
    // TYPES
    
    AnyType
    = ( Number | String | Identifier | Boolean | Function | FunctionBlock )
    
    Boolean
    = true | false
    
    Number
    = digit* "." digit+  -- decimal
    | digit+ -- whole
    
    String
    =  StringID (~StringID any)* StringID
    
    StringID
    = "'" | "\"" | "\`"
    
    Identifier
    = ( letter | "_" ) ( letter | digit | "_" )*
    
    
    // KEYWORDS
    
    Keyword
    = string
    | true
    | false
    | number
    | class
    | array
    | bool
    | write
    | func
    
    string = "string" ~stop
    
    true = "true" ~stop
    
    false = "false" ~stop
    
    number = "number" ~stop
    
    class = "class" ~stop
    
    array = "array" ~stop
    
    bool = "bool" ~stop
    
    write = "write" ~stop
    
    object = "object" ~stop
    
    func = "func" ~stop
    
    stop = "_" | letter | number
    
    
	Comment
    = "***" (~"***" any)* "***" -- multiline
    | "#" ( ~"\n" any)* -- inline
    
}
`)