const GrammarUtils = `

Parens [inner] ~ {
    Surrounded [ inner, '('~, ')'~ ]
}

ParenList [inner, sep] ~ {
    Parens [ List[inner, sep] ]
}

Brackets [inner] ~ {
    Surrounded [ inner, '['~, ']'~ ]
}

BracketList [inner, sep] ~ {
    Brackets [ List[inner, sep] ]
}

Braces [inner] ~ {
    Surrounded [ inner, '{'~, '}'~ ]
}

BracesList [inner, sep] ~ {
    Braces [ List[inner, sep] ]
}

Surrounded [inner, open, close] ~ {
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
    PrimitiveType
    | StructType
    | SumType
    | InstanceType
}

StructType { 'struct'~ BaseType }
SumType { 'sum'~ BaseType }
PrimitiveType { 'primitive'~ PrimitiveName }
PrimitiveName match-strings {
    'int'
    'type'
    'decimal'
    'bool'
    'boo'
    'str'
    'none'
    'env'
}

BaseType ~ {
    BracketList [ FieldType, ','? ]
}

FieldType { Type Ident }

TypeConstructor {
    'type'~ TypeConstructor_Ident '='~ TypeConstructor_Value
}

TypeConstructor_Value ~ {
    (TypeParams '='~ '>'~ Type)
    | Type
}

TypeConstructor_Ident ~ {
    Ident 
    | PrimitiveName
}

TypeParams { List [ Ident, ','~ ] }

InstanceType { Ident InstanceTypeArgs? }
InstanceTypeWithArgs { Ident InstanceTypeArgs }
InstanceTypeArgs { BracketList [ InstanceType, ','~ ] }

ReturnTypeHint ~ { '-'~ '>'~ Type }

Impl {
    'impl'~ Type '='~ ImplBlock
}

ImplBlock ~ {
    BracketList [ ImplFun, ','~ ]
}

ImplFun {
    Ident ':'~ Fun
}

`

export default `

Expr ~ {
    TypeConstructor
    | Impl
    | Block
    | Decl
    | Assign
    | Fun
    | Match
    | IfStmt
    | BinOp
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

Match {
    'match'~ Expr Brackets[MatchBranches]
}

MatchBranches ~ {
    List[MatchBranch, ','~]
    MatchBranch_Default ','?
}

MatchBranch { '_'! Expr ':'~ Expr }
MatchBranch_Default ~ { '_'~ ':'~ Expr }

IfStmt {
    'if'~ Expr Block ElseStmt?
}

ElseStmt ~ {
    'else'~ Block
}

Decl { 'let'~ Decl_Inner '='~ Expr }

Decl_Inner ~ {
    Decl_TypedMut
    | Decl_Mut
    | Decl_Typed
    | Ident
}

Decl_TypedMut {
    'mut'~ Type Ident
}

Decl_Typed {
    Type Ident
}

Decl_Mut {
    'mut'~ Ident
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

Fun { Params ReturnTypeHint? Block }
Params { 
    ParenList[Param, ','~]
    | List[Param, ','~]
}
Param { 
    'self'
    | (Type Ident)
}

BinOp { CompareOp }

CompareTypeOp ~ { CompareOp CompareTypeOp_Right? }
CompareTypeOp_Right ~ { CompareTypeOperator Expr }
CompareTypeOperator match-strings { 'istype' }

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

PowerOp ~ { Call PowerOp_Right? }
PowerOp_Right ~ { PowerOp_Op Expr }
PowerOp_Op match-strings { '^' }

Call { Lookup Args? }
Args { ParenList[Expr, ','~] }

Lookup { UnOp Lookup_Right? }
Lookup_Right ~ { Lookup_Op Lookup_Member }
Lookup_Member ~ { Ident | @string | @int }
Lookup_Op match-strings { '.' '\\\\' }

UnOp { 
    (UnOp_Op Expr) 
    | Group
}

UnOp_Op match-strings { '-' 'not' }

Group ~ {
    Parens[Expr]
    | Leaf
}

Leaf ~ {
    MapLiteral
    | @string
    | @int
    | @decimal
    | InstanceTypeWithArgs
    | Ident
    | Type
    | Bool
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
    'match'
}

Bool {
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