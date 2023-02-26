const GrammarUtils = `

Parens [inner] ~ {
    Bracketed [ inner, '('~, ')'~ ]
}

ParenList [inner, sep] ~ {
    Parens [ List[inner, sep] ]
}

Brackets [inner] ~ {
    Bracketed [ inner, '['~, ']'~ ]
}

BracketList [inner, sep] ~ {
    Brackets [ List[inner, sep] ]
}

Braces [inner] ~ {
    Bracketed [ inner, '{'~, '}'~ ]
}

BracesList [inner, sep] ~ {
    Braces [ List[inner, sep] ]
}

Bracketed [inner, open, close] ~ {
    open~ inner close~
}

List [inner, sep] ~ {
    ListInner[inner, sep]*
    inner?
}

ListInner [inner, sep] ~ {
    inner sep
}

`

const TypeGrammar = `

Type ~ {
    StructType
    | SumType
    | PrimitiveType
    | TypeInstance
}

StructType { 'type'~ '!'~ BaseType }
SumType { 'sum'~ '!'~ BaseType }
PrimitiveType { 'type'~ '!'~ @ident }

BaseType ~ {
    BracketList [ Field, ','? ]
}

TypeHint {
    ':'~ TypeInstance
}

Field { Ident ':'~ FieldValue }

FieldValue ~ {
    Type
    | TypeInstance
}

TypeConstructor {
    TypeConstructor_Ident '='~ TypeConstructor_Value
}

TypeConstructor_Value ~ {
    (TypeParams '='~ '>'~ Type)
    | Type
}

TypeConstructor_Ident ~ {
    'type'~ Ident
}

TypeParams { List [ Ident, ','~ ] }

TypeInstance { Ident TypeInstanceArgs? }

TypeInstanceWithArgs {
    Ident TypeInstanceArgs
}

TypeInstanceArgs { 
    BracketList [ TypeInstance, ','~ ]
}

ReturnTypeHint ~ { '-'~ '>'~ Type }

`

export default `

Expr ~ {
    TypeConstructor
    | Block
    | DeclareMut
    | Declare
    | Assign
    | Func
    | IfStmt
    | BinaryOp
}

${ TypeGrammar }


MapLiteral {
    BracketList[MapField, ','~]
}

MapField {
    MapKey ':'~ Expr
}

MapKey ~ {
    @ident
    | @string 
    | @int
}

IfStmt {
    'if'~ Expr Block ElseStmt?
}

ElseStmt ~ {
    'else'~ Block
}

Declare {
    'let'~ Ident TypeHint? '='~ Expr
}

DeclareMut {
    'let'~ 'mut'~ Ident TypeHint? '='~ Expr
}

Assign {
    Ident '='~ Expr
}

Block {
    BraceBlock
    | ArrowBlock
}

BraceBlock ~ {
    Braces[Expr*]
}

ArrowBlock ~ {
    '='~ '>'~ Expr
}

Func { Params ReturnTypeHint? Block }
Params { 
    ParenList[Param, ','~]
    | List[Param, ','~]
}
Param { Ident TypeHint }

BinaryOp { CompareOp }

CompareOp ~ { AddOp CompareOp_Right? }
CompareOp_Right ~ { CompareOperator Expr }
CompareOperator match-strings {
    'and'
    'or'
    'is'
    'not'
    '<'
    '>'
    '<='
    '>='
    '!='
    '&'
    '|'
}

AddOp ~ { MultiplyOp AddOp_Right? }
AddOp_Right ~ { AddOp_Op Expr }
AddOp_Op match-strings { '+' '-' '%' }

MultiplyOp ~ { PowerOp MultiplyOp_Right? }
MultiplyOp_Right ~ { MultiplyOp_Op Expr }
MultiplyOp_Op match-strings { '*' '/' }

PowerOp ~ { FunCall PowerOp_Right? }
PowerOp_Right ~ { PowerOp_Op Expr }
PowerOp_Op match-strings { '^' }

FunCall { PropertyOp Args? }
Args { ParenList[Expr, ','~] }

PropertyOp ~ { UnaryOp PropertyOp_Right? }
PropertyOp_Right ~ { PropertyOp_Op Expr }
PropertyOp_Op match-strings { '.' '\\\\' }

UnaryOp { 
    ('-' Expr) 
    | Group
}

Group ~ {
    Parens[Expr]
    | Leaf
}

Leaf ~ {
    MapLiteral
    | @string
    | @int
    | @decimal
    | TypeInstanceWithArgs
    | Ident
    | Type
    | Boolean
    | None
}

Reserved match-strings {
    'true'
    'false'
    'none'
    'and'
    'or'
    'not'
    'if'
    'else'
    'for'
    'while'
    'loop'
    'let'
    'mut'
}

Boolean {
    'true'
    | 'false'
}

None { 'none' }

Ident ~ {
    Reserved!
    @ident
}

${ GrammarUtils }

`