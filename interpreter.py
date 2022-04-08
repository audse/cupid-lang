from sly import Lexer, Parser

class CupidLexer ( Lexer ):

    tokens = { 
        ID,
        # ASSIGN, 
        LPAREN, 
        RPAREN,
        WRITE,
        SQUOTE
    }

    ignore = ' \t'

    ID      = r'[a-zA-Z_][a-zA-Z0-9_]*'
    # ASSIGN  = r'='
    LPAREN  = r'\('
    RPAREN  = r'\)'
    SQUOTE  = r'\''

    ID['write'] = WRITE



class CupidParser ( Parser ):
    tokens = CupidLexer.tokens
    
    def __init__(self, variables: dict = None):
        self.variables = variables or {}  # We want to allow input context
        self.stack = []  # We will use a stack to traverse the tree and keep the results

    @property
    def last_item_on_stack(self):
        return self.stack[-1] if len(self.stack) > 0 else None

    @_('LPAREN ID RPAREN')
    def expression(self, p):
        self.stack.append(p)

    @_('WRITE ID')
    def expression(self, p):
        self.stack.append(p)
    
    @_('SQUOTE ID SQUOTE')
    def expression(self, p):
        self.stack.append(p)


lexer = CupidLexer()
while True:
    text = input(">>> ")
    tokens = lexer.tokenize(text) # Creates a generator of tokens
    parser = CupidParser()
    parser.parse(tokens) # The entry point to the parser
    print(parser.last_item_on_stack)