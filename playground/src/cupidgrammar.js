export default `@top File { expression* }

@precedence { 
	TypedDeclaration @left,
	TypeHint @left,
	term @left,
	Struct,
	Sum,
	Function @left,
	Block @left,
	stuctMember,
	identifier @left
}

expression {
	statement
	| term
}

statement {
	TypeDefinition
	| TypedDeclaration
}

term {
	UseBlock
	| UseTraitBlock
	| IfBlock
	| WhileLoop
	| ForInLoop
	| group
	| Map
	| Function
	| Property
	| literal
	| identifier ~path
}

Block {
	BraceBlock
	| ArrowBlock
}

IfBlock { 
	kw<"if"> 
	term 
	Block 
	ElseIfBlock* 
	ElseBlock 
}

ElseIfBlock { kw<"else"> kw<"if"> term Block }
ElseBlock { kw<"else"> Block }

WhileLoop { kw<"while"> term Block }
ForInLoop { kw<"for"> commaSep<identifier> kw<"in"> term Block }

BraceBlock { "{" expression* "}" }
ArrowBlock { Arrow expression }

UseBlock { Use Generics? typeName ~use BraceBlock }
UseTraitBlock { Use Generics? typeName ~use kw<"with"> BraceBlock }
Use { kw<"use"> }

group { "(" expression ")" }

TypeDefinition {
	kw<"type">
	Generics?
	AnyTypeName
	"="
	Type
}

Type {
	Struct
}

Struct { "[" commaSep<structMember> "]" }
structMember { TypeHint StructPropertyName? }
StructPropertyName { identifier }

TypedDeclaration {
	TypeHint  ~path
	kw<"mut">?
	identifier
	"="
	expression
}

Function { commaSep<Parameter> ArrowBlock }
Parameter { TypeHint identifier ~funPath }

Map { "[" commaSep<mapItem> "]" }
mapItem { (PropertyName ":")? expression }
PropertyName { term }

Property { identifier '.' expression }

TypeHint { AnyTypeName ~path ("[" commaSep<typeHintArg> "]")? }

typeHintArg { AnyTypeName (":" AnyTypeName)? }

Generics { unnamedGenerics }

namedGenerics { "[" commaSep<namedGenericPair> "]" }
unnamedGenerics { "[" commaSep<AnyTypeName> "]" }
namedGenericPair { identifier ":" TypeHint }

AnyTypeName { typeName | identifier }

self { @specialize[@name=Self]<identifier, "self"> }
none { @specialize[@name=None]<identifier, "none"> }
boolean { @specialize[@name=Boolean]<identifier, "true" | "false"> }
typeName { @specialize[@name=TypeName]<identifier, 
	"fun"
	| "int"
	| "dec"
	| "array"
	| "map"
	| "char"
	| "bool"
	| "nothing"
	| "string"> ~typePath
}

literal {
	String
	| boolean
	| Integer
	| Decimal
	| self 
	| none
}

@tokens {
	@precedence { Decimal, Integer }
	
	identifier { $[a-zA-Z_]+ }
	String { "'" ![']* "'" }
	Integer { $[0-9]+ }
	Decimal { $[0-9]+ "." $[0-9]+ }
	
	ArithmeticAssignment { "+=" "-=" "*=" "/=" "^=" "%=" }
	ArithmeticOperator { "+" | "-" | "*" | "/" | "^" | "%" }
	CompareOperator { "is" | "istype" | "in" }
	LogicOperator { "and" | "or" | "not" }
	Arrow { "=>" }
	
	space { " " | "\t" | "\n" }
	LineComment { "#" ![\n]* }
	MultiLineComment { 
		stars
		(!stars)*
		stars
	}
	stars { "*" "*" "*" }
}

commaSep<expr> { commaSep1<expr>? }

commaSep1<expr> { expr ("," expr?)* }

kw<term> { @specialize[@name={term}]<identifier, term> }

@skip { space | LineComment | MultiLineComment }
@detectDelim`;
