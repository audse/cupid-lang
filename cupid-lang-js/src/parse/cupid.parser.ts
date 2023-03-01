
import { Option, Node, NodeParser, RuleNode } from '@/types'
import { getNodeArray, node, modifier, makeNode } from '@/parse/utils'
import { TokenParser } from '@/parse/parse'

/*******************************************/
/** AUTOMATICALLY GENERATED - DO NOT EDIT **/
/*******************************************/

export namespace cupid {
    export function expr (parser: TokenParser): Option<Node[]> {
        return getNodeArray(typeconstructor(parser)
            ?? impl(parser)
            ?? block(parser)
            ?? declmut(parser)
            ?? decl(parser)
            ?? assign(parser)
            ?? fun(parser)
            ?? ifstmt(parser)
        ?? binop(parser))
    }
    export function type (parser: TokenParser): Option<Node[]> {
        return getNodeArray(primitivetype(parser)
            ?? structtype(parser)
            ?? sumtype(parser)
        ?? instancetype(parser))
    }
    export function structtype (parser: TokenParser): Option<RuleNode> {
            return makeNode('StructType', parser.chain(
                modifier.passThrough(parser => parser.match('struct')),
                basetype
        ))
    }
    export function sumtype (parser: TokenParser): Option<RuleNode> {
            return makeNode('SumType', parser.chain(
                modifier.passThrough(parser => parser.match('sum')),
                basetype
        ))
    }
    export function primitivetype (parser: TokenParser): Option<RuleNode> {
            return makeNode('PrimitiveType', parser.chain(
                modifier.passThrough(parser => parser.match('primitive')),
                primitivename
        ))
    }
    const primitivenameAccepted: Set<string> = new Set(['int', 'type', 'decimal', 'bool', 'boo', 'str', 'none', 'env'])
    export function primitivename (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(primitivenameAccepted))
    }
    export function basetype (parser: TokenParser): Option<Node[]> {
            return getNodeArray(bracketlist(
                parser,
                fieldtype,
                modifier.optional(parser => node.string(parser.match(',')))
        ))
    }
    export function typehint (parser: TokenParser): Option<RuleNode> {
            return makeNode('TypeHint', parser.chain(
                modifier.passThrough(parser => parser.match(':')),
                instancetype
        ))
    }
    export function fieldtype (parser: TokenParser): Option<RuleNode> {
            return makeNode('FieldType', parser.chain(
                ident,
                modifier.passThrough(parser => parser.match(':')),
                type
        ))
    }
    export function typeconstructor (parser: TokenParser): Option<RuleNode> {
            return makeNode('TypeConstructor', parser.chain(
                modifier.passThrough(parser => parser.match('type')),
                typeconstructor_ident,
                modifier.passThrough(parser => parser.match('=')),
                typeconstructor_value
        ))
    }
    export function typeconstructor_value (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                typeparams,
                modifier.passThrough(parser => parser.match('=')),
                modifier.passThrough(parser => parser.match('>')),
                type
            )
        ?? type(parser))
    }
    export function typeconstructor_ident (parser: TokenParser): Option<Node[]> {
        return getNodeArray(ident(parser)
        ?? primitivename(parser))
    }
    export function typeparams (parser: TokenParser): Option<RuleNode> {
            return makeNode('TypeParams', list(
                parser,
                ident,
                modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function instancetype (parser: TokenParser): Option<RuleNode> {
            return makeNode('InstanceType', parser.chain(
                ident,
                modifier.optional(instancetypeargs)
        ))
    }
    export function instancetypewithargs (parser: TokenParser): Option<RuleNode> {
            return makeNode('InstanceTypeWithArgs', parser.chain(
                ident,
                instancetypeargs
        ))
    }
    export function instancetypeargs (parser: TokenParser): Option<RuleNode> {
            return makeNode('InstanceTypeArgs', bracketlist(
                parser,
                instancetype,
                modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function returntypehint (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                modifier.passThrough(parser => parser.match('-')),
                modifier.passThrough(parser => parser.match('>')),
                type
        ))
    }
    export function impl (parser: TokenParser): Option<RuleNode> {
            return makeNode('Impl', parser.chain(
                modifier.passThrough(parser => parser.match('impl')),
                type,
                modifier.passThrough(parser => parser.match('=')),
                implblock
        ))
    }
    export function implblock (parser: TokenParser): Option<Node[]> {
            return getNodeArray(bracketlist(
                parser,
                implfun,
                modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function implfun (parser: TokenParser): Option<RuleNode> {
            return makeNode('ImplFun', parser.chain(
                ident,
                modifier.passThrough(parser => parser.match(':')),
                fun
        ))
    }
    export function mapliteral (parser: TokenParser): Option<RuleNode> {
            return makeNode('MapLiteral', bracketlist(
                parser,
                mapfield,
                modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function mapfield (parser: TokenParser): Option<RuleNode> {
            return makeNode('MapField', parser.chain(
                mapkey,
                modifier.passThrough(parser => parser.match(':')),
                expr
        ))
    }
    export function mapkey (parser: TokenParser): Option<Node[]> {
        return getNodeArray(node.ident(parser.ident())
            ?? node.string(parser.string())
        ?? node.int(parser.int()))
    }
    export function ifstmt (parser: TokenParser): Option<RuleNode> {
            return makeNode('IfStmt', parser.chain(
                modifier.passThrough(parser => parser.match('if')),
                expr,
                block,
                modifier.optional(elsestmt)
        ))
    }
    export function elsestmt (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                modifier.passThrough(parser => parser.match('else')),
                block
        ))
    }
    export function decl (parser: TokenParser): Option<RuleNode> {
            return makeNode('Decl', parser.chain(
                modifier.passThrough(parser => parser.match('let')),
                ident,
                modifier.optional(typehint),
                modifier.passThrough(parser => parser.match('=')),
                expr
        ))
    }
    export function declmut (parser: TokenParser): Option<RuleNode> {
            return makeNode('DeclMut', parser.chain(
                modifier.passThrough(parser => parser.match('let')),
                modifier.passThrough(parser => parser.match('mut')),
                ident,
                modifier.optional(typehint),
                modifier.passThrough(parser => parser.match('=')),
                expr
        ))
    }
    export function assign (parser: TokenParser): Option<RuleNode> {
            return makeNode('Assign', parser.chain(
                ident,
                modifier.passThrough(parser => parser.match('=')),
                expr
        ))
    }
    export function block (parser: TokenParser): Option<RuleNode> {
        return makeNode('Block', braceblock(parser)
        ?? arrowblock(parser))
    }
    export function braceblock (parser: TokenParser): Option<Node[]> {
            return getNodeArray(braces(
                parser,
                modifier.multiple(expr)
        ))
    }
    export function arrowblock (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                modifier.passThrough(parser => parser.match('=')),
                modifier.passThrough(parser => parser.match('>')),
                expr
        ))
    }
    export function fun (parser: TokenParser): Option<RuleNode> {
            return makeNode('Fun', parser.chain(
                params,
                modifier.optional(returntypehint),
                block
        ))
    }
    export function params (parser: TokenParser): Option<RuleNode> {
            return makeNode('Params', parenlist(
                parser,
                param,
                modifier.passThrough(parser => parser.match(','))
            )
            ?? list(
                parser,
                param,
                modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function param (parser: TokenParser): Option<RuleNode> {
        return makeNode('Param', node.string(parser.match('self'))
            ?? parser.chain(
                ident,
                typehint
        ))
    }
    export function binop (parser: TokenParser): Option<RuleNode> {
        return makeNode('BinOp', compareop(parser))
    }
    export function compareop (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                addop,
                modifier.optional(compareop_right)
        ))
    }
    export function compareop_right (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                compareoperator,
                expr
        ))
    }
    const compareoperatorAccepted: Set<string> = new Set(['and', 'or', 'is', 'not', '<', '>', '<=', '>=', '!=', '&', '|'])
    export function compareoperator (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(compareoperatorAccepted))
    }
    export function addop (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                multiplyop,
                modifier.optional(addop_right)
        ))
    }
    export function addop_right (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                addop_op,
                expr
        ))
    }
    const addop_opAccepted: Set<string> = new Set(['+', '-', '%'])
    export function addop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(addop_opAccepted))
    }
    export function multiplyop (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                powerop,
                modifier.optional(multiplyop_right)
        ))
    }
    export function multiplyop_right (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                multiplyop_op,
                expr
        ))
    }
    const multiplyop_opAccepted: Set<string> = new Set(['*', '/'])
    export function multiplyop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(multiplyop_opAccepted))
    }
    export function powerop (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                call,
                modifier.optional(powerop_right)
        ))
    }
    export function powerop_right (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                powerop_op,
                expr
        ))
    }
    const powerop_opAccepted: Set<string> = new Set(['^'])
    export function powerop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(powerop_opAccepted))
    }
    export function call (parser: TokenParser): Option<RuleNode> {
            return makeNode('Call', parser.chain(
                lookup,
                modifier.optional(args)
        ))
    }
    export function args (parser: TokenParser): Option<RuleNode> {
            return makeNode('Args', parenlist(
                parser,
                expr,
                modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function lookup (parser: TokenParser): Option<RuleNode> {
            return makeNode('Lookup', parser.chain(
                unop,
                modifier.optional(lookup_right)
        ))
    }
    export function lookup_right (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                lookup_op,
                lookup_member
        ))
    }
    export function lookup_member (parser: TokenParser): Option<Node[]> {
        return getNodeArray(ident(parser)
            ?? node.string(parser.string())
        ?? node.int(parser.int()))
    }
    const lookup_opAccepted: Set<string> = new Set(['.', '\\'])
    export function lookup_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(lookup_opAccepted))
    }
    export function unop (parser: TokenParser): Option<RuleNode> {
            return makeNode('UnOp', parser.chain(
                parser => node.string(parser.match('-')),
                expr
            )
        ?? group(parser))
    }
    export function group (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parens(
                parser,
                expr
            )
        ?? leaf(parser))
    }
    export function leaf (parser: TokenParser): Option<Node[]> {
        return getNodeArray(mapliteral(parser)
            ?? node.string(parser.string())
            ?? node.int(parser.int())
            ?? node.decimal(parser.decimal())
            ?? instancetypewithargs(parser)
            ?? ident(parser)
            ?? type(parser)
            ?? bool(parser)
        ?? none(parser))
    }
    const reservedAccepted: Set<string> = new Set(['true', 'false', 'none', 'and', 'or', 'not', 'if', 'else', 'for', 'while', 'loop', 'let', 'mut'])
    export function reserved (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(reservedAccepted))
    }
    export function bool (parser: TokenParser): Option<RuleNode> {
        return makeNode('Bool', node.string(parser.match('true'))
        ?? node.string(parser.match('false')))
    }
    export function none (parser: TokenParser): Option<RuleNode> {
        return makeNode('None', node.string(parser.match('none')))
    }
    export function ident (parser: TokenParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                modifier.negative(reserved),
                parser => node.ident(parser.ident())
        ))
    }
    export function parens (parser: TokenParser, inner: NodeParser): Option<Node[]> {
            return getNodeArray(surrounded(
                parser,
                inner,
                modifier.passThrough(parser => parser.match('(')),
                modifier.passThrough(parser => parser.match(')'))
        ))
    }
    export function parenlist (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            return getNodeArray(parens(
                parser,
                parser => list(
                    parser,
                    inner,
                    sep
                )
        ))
    }
    export function brackets (parser: TokenParser, inner: NodeParser): Option<Node[]> {
            return getNodeArray(surrounded(
                parser,
                inner,
                modifier.passThrough(parser => parser.match('[')),
                modifier.passThrough(parser => parser.match(']'))
        ))
    }
    export function bracketlist (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            return getNodeArray(brackets(
                parser,
                parser => list(
                    parser,
                    inner,
                    sep
                )
        ))
    }
    export function braces (parser: TokenParser, inner: NodeParser): Option<Node[]> {
            return getNodeArray(surrounded(
                parser,
                inner,
                modifier.passThrough(parser => parser.match('{')),
                modifier.passThrough(parser => parser.match('}'))
        ))
    }
    export function braceslist (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            return getNodeArray(braces(
                parser,
                parser => list(
                    parser,
                    inner,
                    sep
                )
        ))
    }
    export function surrounded (parser: TokenParser, inner: NodeParser, open: NodeParser, close: NodeParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                modifier.passThrough(open),
                inner,
                modifier.passThrough(close)
        ))
    }
    export function list (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                    modifier.multiple(parser => listinner(
                        parser,
                        inner,
                        sep
                )),
                modifier.optional(inner)
        ))
    }
    export function listinner (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            return getNodeArray(parser.chain(
                inner,
                sep
        ))
    }
}