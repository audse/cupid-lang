
import { Option, token, Node, NodeParser, CustomNode } from '@/types'
import { getNodeArray, node, modifier, makeNode } from '@/parse/utils'
import { TokenParser } from '@/parse/parse'

/*******************************************/
/** AUTOMATICALLY GENERATED - DO NOT EDIT **/
/*******************************************/

export namespace cupid {
    export function expr (parser: TokenParser): Option<Node[]> {
        return getNodeArray(typeconstructor(parser)
            ?? block(parser)
            ?? declaremut(parser)
            ?? declare(parser)
            ?? assign(parser)
            ?? func(parser)
            ?? ifstmt(parser)
            ?? binaryop(parser))
    }
    export function type (parser: TokenParser): Option<Node[]> {
        return getNodeArray(structtype(parser)
            ?? sumtype(parser)
            ?? primitivetype(parser)
            ?? typeinstance(parser))
    }
    export function structtype (parser: TokenParser): Option<CustomNode> {
        return makeNode('StructType', parser.chain(
            modifier.passThrough(parser => parser.match('type')),
            modifier.passThrough(parser => parser.match('!')),
            basetype
        ))
    }
    export function sumtype (parser: TokenParser): Option<CustomNode> {
        return makeNode('SumType', parser.chain(
            modifier.passThrough(parser => parser.match('sum')),
            modifier.passThrough(parser => parser.match('!')),
            basetype
        ))
    }
    export function primitivetype (parser: TokenParser): Option<CustomNode> {
        return makeNode('PrimitiveType', parser.chain(
            modifier.passThrough(parser => parser.match('type')),
            modifier.passThrough(parser => parser.match('!')),
            parser => node.ident(parser.ident())
        ))
    }
    export function basetype (parser: TokenParser): Option<Node[]> {
        return getNodeArray(bracketlist(
            parser,
            field,
            modifier.optional(parser => node.string(parser.match(',')))
        ))
    }
    export function typehint (parser: TokenParser): Option<CustomNode> {
        return makeNode('TypeHint', parser.chain(
            modifier.passThrough(parser => parser.match(':')),
            typeinstance
        ))
    }
    export function field (parser: TokenParser): Option<CustomNode> {
        return makeNode('Field', parser.chain(
            ident,
            modifier.passThrough(parser => parser.match(':')),
            fieldvalue
        ))
    }
    export function fieldvalue (parser: TokenParser): Option<Node[]> {
        return getNodeArray(type(parser)
            ?? typeinstance(parser))
    }
    export function typeconstructor (parser: TokenParser): Option<CustomNode> {
        return makeNode('TypeConstructor', parser.chain(
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
        return getNodeArray(parser.chain(
            modifier.passThrough(parser => parser.match('type')),
            ident
        ))
    }
    export function typeparams (parser: TokenParser): Option<CustomNode> {
        return makeNode('TypeParams', list(
            parser,
            ident,
            modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function typeinstance (parser: TokenParser): Option<CustomNode> {
        return makeNode('TypeInstance', parser.chain(
            ident,
            modifier.optional(typeinstanceargs)
        ))
    }
    export function typeinstancewithargs (parser: TokenParser): Option<CustomNode> {
        return makeNode('TypeInstanceWithArgs', parser.chain(
            ident,
            typeinstanceargs
        ))
    }
    export function typeinstanceargs (parser: TokenParser): Option<CustomNode> {
        return makeNode('TypeInstanceArgs', bracketlist(
            parser,
            typeinstance,
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
    export function mapliteral (parser: TokenParser): Option<CustomNode> {
        return makeNode('MapLiteral', bracketlist(
            parser,
            mapfield,
            modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function mapfield (parser: TokenParser): Option<CustomNode> {
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
    export function ifstmt (parser: TokenParser): Option<CustomNode> {
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
    export function declare (parser: TokenParser): Option<CustomNode> {
        return makeNode('Declare', parser.chain(
            modifier.passThrough(parser => parser.match('let')),
            ident,
            modifier.optional(typehint),
            modifier.passThrough(parser => parser.match('=')),
            expr
        ))
    }
    export function declaremut (parser: TokenParser): Option<CustomNode> {
        return makeNode('DeclareMut', parser.chain(
            modifier.passThrough(parser => parser.match('let')),
            modifier.passThrough(parser => parser.match('mut')),
            ident,
            modifier.optional(typehint),
            modifier.passThrough(parser => parser.match('=')),
            expr
        ))
    }
    export function assign (parser: TokenParser): Option<CustomNode> {
        return makeNode('Assign', parser.chain(
            ident,
            modifier.passThrough(parser => parser.match('=')),
            expr
        ))
    }
    export function block (parser: TokenParser): Option<CustomNode> {
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
    export function func (parser: TokenParser): Option<CustomNode> {
        return makeNode('Func', parser.chain(
            params,
            modifier.optional(returntypehint),
            block
        ))
    }
    export function params (parser: TokenParser): Option<CustomNode> {
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
    export function param (parser: TokenParser): Option<CustomNode> {
        return makeNode('Param', parser.chain(
            ident,
            typehint
        ))
    }
    export function binaryop (parser: TokenParser): Option<CustomNode> {
        return makeNode('BinaryOp', compareop(parser))
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
            funcall,
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
    export function funcall (parser: TokenParser): Option<CustomNode> {
        return makeNode('FunCall', parser.chain(
            propertyop,
            modifier.optional(args)
        ))
    }
    export function args (parser: TokenParser): Option<CustomNode> {
        return makeNode('Args', parenlist(
            parser,
            expr,
            modifier.passThrough(parser => parser.match(','))
        ))
    }
    export function propertyop (parser: TokenParser): Option<Node[]> {
        return getNodeArray(parser.chain(
            unaryop,
            modifier.optional(propertyop_right)
        ))
    }
    export function propertyop_right (parser: TokenParser): Option<Node[]> {
        return getNodeArray(parser.chain(
            propertyop_op,
            expr
        ))
    }
    const propertyop_opAccepted: Set<string> = new Set(['.', '\\'])
    export function propertyop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(propertyop_opAccepted))
    }
    export function unaryop (parser: TokenParser): Option<CustomNode> {
        return makeNode('UnaryOp', parser.chain(
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
            ?? typeinstancewithargs(parser)
            ?? ident(parser)
            ?? type(parser)
            ?? boolean(parser)
            ?? none(parser))
    }
    const reservedAccepted: Set<string> = new Set(['true', 'false', 'none', 'and', 'or', 'not', 'if', 'else', 'for', 'while', 'loop', 'let', 'mut'])
    export function reserved (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(reservedAccepted))
    }
    export function boolean (parser: TokenParser): Option<CustomNode> {
        return makeNode('Boolean', node.string(parser.match('true'))
            ?? node.string(parser.match('false')))
    }
    export function none (parser: TokenParser): Option<CustomNode> {
        return makeNode('None', node.string(parser.match('none')))
    }
    export function ident (parser: TokenParser): Option<Node[]> {
        return getNodeArray(parser.chain(
            modifier.negative(reserved),
            parser => node.ident(parser.ident())
        ))
    }
    export function parens (parser: TokenParser, inner: NodeParser): Option<Node[]> {
        return getNodeArray(bracketed(
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
        return getNodeArray(bracketed(
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
        return getNodeArray(bracketed(
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
    export function bracketed (parser: TokenParser, inner: NodeParser, open: NodeParser, close: NodeParser): Option<Node[]> {
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