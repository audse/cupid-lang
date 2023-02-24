
import { Option, token, Node, NodeParser } from '@/types'
import { getNodeArray, node, modifier } from '@/parse/utils'
import { TokenParser } from '@/parse/parse'

/*******************************************/
/** AUTOMATICALLY GENERATED - DO NOT EDIT **/
/*******************************************/

export namespace cupid {
    export function expr (parser: TokenParser): Option<Node[]> {
        const exprGroup = ((parser): Option<Node | Node[]> => typeconstructor(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => block(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => declaremut(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => declare(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => func(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => ifstmt(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => assign(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => binaryop(parser))(parser)
        if (exprGroup) return getNodeArray(exprGroup)
        return null
    }
    export function type (parser: TokenParser): Option<Node[]> {
        const typeGroup = ((parser): Option<Node | Node[]> => structtype(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => sumtype(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => primitivetype(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => typeinstance(parser))(parser)
        if (typeGroup) return getNodeArray(typeGroup)
        return null
    }
    export function structtype (parser: TokenParser): Option<Node> {
        const structtypeGroup = parser.chain(
            (parser): Option<boolean> => parser.match('type') ? true : null,
            (parser): Option<boolean> => parser.match('!') ? true : null,
            (parser): Option<Node | Node[]> => basetype(parser)
        )
        if (structtypeGroup) return {
            name: 'StructType',
            items: getNodeArray(structtypeGroup),
        }
        return null
    }
    export function sumtype (parser: TokenParser): Option<Node> {
        const sumtypeGroup = parser.chain(
            (parser): Option<boolean> => parser.match('sum') ? true : null,
            (parser): Option<boolean> => parser.match('!') ? true : null,
            (parser): Option<Node | Node[]> => basetype(parser)
        )
        if (sumtypeGroup) return {
            name: 'SumType',
            items: getNodeArray(sumtypeGroup),
        }
        return null
    }
    export function primitivetype (parser: TokenParser): Option<Node> {
        const primitivetypeGroup = parser.chain(
            (parser): Option<boolean> => parser.match('type') ? true : null,
            (parser): Option<boolean> => parser.match('!') ? true : null,
            (parser): Option<Node | Node[]> => node.ident(parser.ident())
        )
        if (primitivetypeGroup) return {
            name: 'PrimitiveType',
            items: getNodeArray(primitivetypeGroup),
        }
        return null
    }
    export function basetype (parser: TokenParser): Option<Node[]> {
            const basetypeGroup = ((parser): Option<Node | Node[]> => bracketlist(
                parser,
                (parser): Option<Node | Node[]> => field(parser),
                parser => modifier.optional(parser, (parser): Option<Node | Node[]> => node.string(parser.match(',')))
        ))(parser)
        if (basetypeGroup) return getNodeArray(basetypeGroup)
        return null
    }
    export function typehint (parser: TokenParser): Option<Node> {
        const typehintGroup = parser.chain(
            (parser): Option<boolean> => parser.match(':') ? true : null,
            (parser): Option<Node | Node[]> => typeinstance(parser)
        )
        if (typehintGroup) return {
            name: 'TypeHint',
            items: getNodeArray(typehintGroup),
        }
        return null
    }
    export function field (parser: TokenParser): Option<Node> {
        const fieldGroup = parser.chain(
            (parser): Option<Node | Node[]> => ident(parser),
            (parser): Option<boolean> => parser.match(':') ? true : null,
            (parser): Option<Node | Node[]> => fieldvalue(parser)
        )
        if (fieldGroup) return {
            name: 'Field',
            items: getNodeArray(fieldGroup),
        }
        return null
    }
    export function fieldvalue (parser: TokenParser): Option<Node[]> {
        const fieldvalueGroup = ((parser): Option<Node | Node[]> => type(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => typeinstance(parser))(parser)
        if (fieldvalueGroup) return getNodeArray(fieldvalueGroup)
        return null
    }
    export function typeconstructor (parser: TokenParser): Option<Node> {
        const typeconstructorGroup = parser.chain(
            (parser): Option<Node | Node[]> => typeconstructor_ident(parser),
            (parser): Option<boolean> => parser.match('=') ? true : null,
            (parser): Option<Node | Node[]> => typeconstructor_value(parser)
        )
        if (typeconstructorGroup) return {
            name: 'TypeConstructor',
            items: getNodeArray(typeconstructorGroup),
        }
        return null
    }
    export function typeconstructor_value (parser: TokenParser): Option<Node[]> {
        const typeconstructor_valueGroup = parser.chain(
            (parser): Option<Node | Node[]> => typeparams(parser),
            (parser): Option<boolean> => parser.match('=') ? true : null,
            (parser): Option<boolean> => parser.match('>') ? true : null,
            (parser): Option<Node | Node[]> => type(parser)
        )
        ?? ((parser): Option<Node | Node[]> => type(parser))(parser)
        if (typeconstructor_valueGroup) return getNodeArray(typeconstructor_valueGroup)
        return null
    }
    export function typeconstructor_ident (parser: TokenParser): Option<Node[]> {
        const typeconstructor_identGroup = parser.chain(
            (parser): Option<boolean> => parser.match('type') ? true : null,
            (parser): Option<Node | Node[]> => ident(parser)
        )
        if (typeconstructor_identGroup) return getNodeArray(typeconstructor_identGroup)
        return null
    }
    export function typeparams (parser: TokenParser): Option<Node> {
            const typeparamsGroup = ((parser): Option<Node | Node[]> => list(
                parser,
                (parser): Option<Node | Node[]> => ident(parser),
                (parser): Option<boolean> => parser.match(',') ? true : null
        ))(parser)
        if (typeparamsGroup) return {
            name: 'TypeParams',
            items: getNodeArray(typeparamsGroup),
        }
        return null
    }
    export function typeinstance (parser: TokenParser): Option<Node> {
        const typeinstanceGroup = parser.chain(
            (parser): Option<Node | Node[]> => ident(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => typeinstanceargs(parser))
        )
        if (typeinstanceGroup) return {
            name: 'TypeInstance',
            items: getNodeArray(typeinstanceGroup),
        }
        return null
    }
    export function typeinstancewithargs (parser: TokenParser): Option<Node> {
        const typeinstancewithargsGroup = parser.chain(
            (parser): Option<Node | Node[]> => ident(parser),
            (parser): Option<Node | Node[]> => typeinstanceargs(parser)
        )
        if (typeinstancewithargsGroup) return {
            name: 'TypeInstanceWithArgs',
            items: getNodeArray(typeinstancewithargsGroup),
        }
        return null
    }
    export function typeinstanceargs (parser: TokenParser): Option<Node> {
            const typeinstanceargsGroup = ((parser): Option<Node | Node[]> => bracketlist(
                parser,
                (parser): Option<Node | Node[]> => typeinstance(parser),
                (parser): Option<boolean> => parser.match(',') ? true : null
        ))(parser)
        if (typeinstanceargsGroup) return {
            name: 'TypeInstanceArgs',
            items: getNodeArray(typeinstanceargsGroup),
        }
        return null
    }
    export function returntypehint (parser: TokenParser): Option<Node[]> {
        const returntypehintGroup = parser.chain(
            (parser): Option<boolean> => parser.match('-') ? true : null,
            (parser): Option<boolean> => parser.match('>') ? true : null,
            (parser): Option<Node | Node[]> => type(parser)
        )
        if (returntypehintGroup) return getNodeArray(returntypehintGroup)
        return null
    }
    export function mapliteral (parser: TokenParser): Option<Node> {
            const mapliteralGroup = ((parser): Option<Node | Node[]> => bracketlist(
                parser,
                (parser): Option<Node | Node[]> => mapfield(parser),
                (parser): Option<boolean> => parser.match(',') ? true : null
        ))(parser)
        if (mapliteralGroup) return {
            name: 'MapLiteral',
            items: getNodeArray(mapliteralGroup),
        }
        return null
    }
    export function mapfield (parser: TokenParser): Option<Node> {
        const mapfieldGroup = parser.chain(
            (parser): Option<Node | Node[]> => mapkey(parser),
            (parser): Option<boolean> => parser.match(':') ? true : null,
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (mapfieldGroup) return {
            name: 'MapField',
            items: getNodeArray(mapfieldGroup),
        }
        return null
    }
    export function mapkey (parser: TokenParser): Option<Node[]> {
        const mapkeyGroup = ((parser): Option<Node | Node[]> => node.ident(parser.ident()))(parser)
        ?? ((parser): Option<Node | Node[]> => node.string(parser.string()))(parser)
        ?? ((parser): Option<Node | Node[]> => node.int(parser.int()))(parser)
        if (mapkeyGroup) return getNodeArray(mapkeyGroup)
        return null
    }
    export function ifstmt (parser: TokenParser): Option<Node> {
        const ifstmtGroup = parser.chain(
            (parser): Option<boolean> => parser.match('if') ? true : null,
            (parser): Option<Node | Node[]> => expr(parser),
            (parser): Option<Node | Node[]> => block(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => elsestmt(parser))
        )
        if (ifstmtGroup) return {
            name: 'IfStmt',
            items: getNodeArray(ifstmtGroup),
        }
        return null
    }
    export function elsestmt (parser: TokenParser): Option<Node[]> {
        const elsestmtGroup = parser.chain(
            (parser): Option<boolean> => parser.match('else') ? true : null,
            (parser): Option<Node | Node[]> => block(parser)
        )
        if (elsestmtGroup) return getNodeArray(elsestmtGroup)
        return null
    }
    export function declare (parser: TokenParser): Option<Node> {
        const declareGroup = parser.chain(
            (parser): Option<boolean> => parser.match('let') ? true : null,
            (parser): Option<Node | Node[]> => ident(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => typehint(parser)),
            (parser): Option<boolean> => parser.match('=') ? true : null,
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (declareGroup) return {
            name: 'Declare',
            items: getNodeArray(declareGroup),
        }
        return null
    }
    export function declaremut (parser: TokenParser): Option<Node> {
        const declaremutGroup = parser.chain(
            (parser): Option<boolean> => parser.match('let') ? true : null,
            (parser): Option<boolean> => parser.match('mut') ? true : null,
            (parser): Option<Node | Node[]> => ident(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => typehint(parser)),
            (parser): Option<boolean> => parser.match('=') ? true : null,
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (declaremutGroup) return {
            name: 'DeclareMut',
            items: getNodeArray(declaremutGroup),
        }
        return null
    }
    export function assign (parser: TokenParser): Option<Node> {
        const assignGroup = parser.chain(
            (parser): Option<Node | Node[]> => ident(parser),
            (parser): Option<boolean> => parser.match('=') ? true : null,
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (assignGroup) return {
            name: 'Assign',
            items: getNodeArray(assignGroup),
        }
        return null
    }
    export function block (parser: TokenParser): Option<Node> {
        const blockGroup = ((parser): Option<Node | Node[]> => braceblock(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => arrowblock(parser))(parser)
        if (blockGroup) return {
            name: 'Block',
            items: getNodeArray(blockGroup),
        }
        return null
    }
    export function braceblock (parser: TokenParser): Option<Node[]> {
            const braceblockGroup = ((parser): Option<Node | Node[]> => braces(
                parser,
                parser => modifier.multiple(parser, (parser): Option<Node | Node[]> => expr(parser)).flat()
        ))(parser)
        if (braceblockGroup) return getNodeArray(braceblockGroup)
        return null
    }
    export function arrowblock (parser: TokenParser): Option<Node[]> {
        const arrowblockGroup = parser.chain(
            (parser): Option<boolean> => parser.match('=') ? true : null,
            (parser): Option<boolean> => parser.match('>') ? true : null,
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (arrowblockGroup) return getNodeArray(arrowblockGroup)
        return null
    }
    export function func (parser: TokenParser): Option<Node> {
        const funcGroup = parser.chain(
            (parser): Option<Node | Node[]> => params(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => returntypehint(parser)),
            (parser): Option<Node | Node[]> => block(parser)
        )
        if (funcGroup) return {
            name: 'Func',
            items: getNodeArray(funcGroup),
        }
        return null
    }
    export function params (parser: TokenParser): Option<Node> {
            const paramsGroup = ((parser): Option<Node | Node[]> => parenlist(
                parser,
                (parser): Option<Node | Node[]> => param(parser),
                (parser): Option<boolean> => parser.match(',') ? true : null
        ))(parser)
            ?? ((parser): Option<Node | Node[]> => list(
                parser,
                (parser): Option<Node | Node[]> => param(parser),
                (parser): Option<boolean> => parser.match(',') ? true : null
        ))(parser)
        if (paramsGroup) return {
            name: 'Params',
            items: getNodeArray(paramsGroup),
        }
        return null
    }
    export function param (parser: TokenParser): Option<Node> {
        const paramGroup = parser.chain(
            (parser): Option<Node | Node[]> => ident(parser),
            (parser): Option<Node | Node[]> => typehint(parser)
        )
        if (paramGroup) return {
            name: 'Param',
            items: getNodeArray(paramGroup),
        }
        return null
    }
    export function funcall (parser: TokenParser): Option<Node> {
        const funcallGroup = parser.chain(
            (parser): Option<Node | Node[]> => funcall_fun(parser),
            (parser): Option<Node | Node[]> => args(parser)
        )
        if (funcallGroup) return {
            name: 'FunCall',
            items: getNodeArray(funcallGroup),
        }
        return null
    }
    export function funcall_fun (parser: TokenParser): Option<Node[]> {
            const funcall_funGroup = ((parser): Option<Node | Node[]> => parens(
                parser,
                (parser): Option<Node | Node[]> => expr(parser)
        ))(parser)
        ?? ((parser): Option<Node | Node[]> => ident(parser))(parser)
        if (funcall_funGroup) return getNodeArray(funcall_funGroup)
        return null
    }
    export function args (parser: TokenParser): Option<Node> {
            const argsGroup = ((parser): Option<Node | Node[]> => parenlist(
                parser,
                (parser): Option<Node | Node[]> => expr(parser),
                (parser): Option<boolean> => parser.match(',') ? true : null
        ))(parser)
        if (argsGroup) return {
            name: 'Args',
            items: getNodeArray(argsGroup),
        }
        return null
    }
    export function binaryop (parser: TokenParser): Option<Node> {
        const binaryopGroup = ((parser): Option<Node | Node[]> => compareop(parser))(parser)
        if (binaryopGroup) return {
            name: 'BinaryOp',
            items: getNodeArray(binaryopGroup),
        }
        return null
    }
    export function compareop (parser: TokenParser): Option<Node[]> {
        const compareopGroup = parser.chain(
            (parser): Option<Node | Node[]> => addop(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => compareop_right(parser))
        )
        if (compareopGroup) return getNodeArray(compareopGroup)
        return null
    }
    export function compareop_right (parser: TokenParser): Option<Node[]> {
        const compareop_rightGroup = parser.chain(
            (parser): Option<Node | Node[]> => compareoperator(parser),
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (compareop_rightGroup) return getNodeArray(compareop_rightGroup)
        return null
    }
    const compareoperatorAccepted: Set<string> = new Set(['and', 'or', 'is', 'not', '<', '>', '<=', '>=', '!=', '&', '|'])
    export function compareoperator (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(compareoperatorAccepted))
    }
    export function addop (parser: TokenParser): Option<Node[]> {
        const addopGroup = parser.chain(
            (parser): Option<Node | Node[]> => multiplyop(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => addop_right(parser))
        )
        if (addopGroup) return getNodeArray(addopGroup)
        return null
    }
    export function addop_right (parser: TokenParser): Option<Node[]> {
        const addop_rightGroup = parser.chain(
            (parser): Option<Node | Node[]> => addop_op(parser),
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (addop_rightGroup) return getNodeArray(addop_rightGroup)
        return null
    }
    const addop_opAccepted: Set<string> = new Set(['+', '-'])
    export function addop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(addop_opAccepted))
    }
    export function multiplyop (parser: TokenParser): Option<Node[]> {
        const multiplyopGroup = parser.chain(
            (parser): Option<Node | Node[]> => powerop(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => multiplyop_right(parser))
        )
        if (multiplyopGroup) return getNodeArray(multiplyopGroup)
        return null
    }
    export function multiplyop_right (parser: TokenParser): Option<Node[]> {
        const multiplyop_rightGroup = parser.chain(
            (parser): Option<Node | Node[]> => multiplyop_op(parser),
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (multiplyop_rightGroup) return getNodeArray(multiplyop_rightGroup)
        return null
    }
    const multiplyop_opAccepted: Set<string> = new Set(['*', '/'])
    export function multiplyop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(multiplyop_opAccepted))
    }
    export function powerop (parser: TokenParser): Option<Node[]> {
        const poweropGroup = parser.chain(
            (parser): Option<Node | Node[]> => propertyop(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => powerop_right(parser))
        )
        if (poweropGroup) return getNodeArray(poweropGroup)
        return null
    }
    export function powerop_right (parser: TokenParser): Option<Node[]> {
        const powerop_rightGroup = parser.chain(
            (parser): Option<Node | Node[]> => powerop_op(parser),
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (powerop_rightGroup) return getNodeArray(powerop_rightGroup)
        return null
    }
    const powerop_opAccepted: Set<string> = new Set(['^'])
    export function powerop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(powerop_opAccepted))
    }
    export function propertyop (parser: TokenParser): Option<Node[]> {
        const propertyopGroup = parser.chain(
            (parser): Option<Node | Node[]> => unaryop(parser),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => propertyop_right(parser))
        )
        if (propertyopGroup) return getNodeArray(propertyopGroup)
        return null
    }
    export function propertyop_right (parser: TokenParser): Option<Node[]> {
        const propertyop_rightGroup = parser.chain(
            (parser): Option<Node | Node[]> => propertyop_op(parser),
            (parser): Option<Node | Node[]> => expr(parser)
        )
        if (propertyop_rightGroup) return getNodeArray(propertyop_rightGroup)
        return null
    }
    const propertyop_opAccepted: Set<string> = new Set(['.', '\\'])
    export function propertyop_op (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(propertyop_opAccepted))
    }
    export function unaryop (parser: TokenParser): Option<Node> {
        const unaryopGroup = parser.chain(
            (parser): Option<Node | Node[]> => node.string(parser.match('-')),
            (parser): Option<Node | Node[]> => expr(parser)
        )
        ?? ((parser): Option<Node | Node[]> => group(parser))(parser)
        if (unaryopGroup) return {
            name: 'UnaryOp',
            items: getNodeArray(unaryopGroup),
        }
        return null
    }
    export function group (parser: TokenParser): Option<Node[]> {
            const groupGroup = ((parser): Option<Node | Node[]> => parens(
                parser,
                (parser): Option<Node | Node[]> => expr(parser)
        ))(parser)
        ?? ((parser): Option<Node | Node[]> => leaf(parser))(parser)
        if (groupGroup) return getNodeArray(groupGroup)
        return null
    }
    export function leaf (parser: TokenParser): Option<Node[]> {
        const leafGroup = ((parser): Option<Node | Node[]> => mapliteral(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => node.string(parser.string()))(parser)
        ?? ((parser): Option<Node | Node[]> => node.int(parser.int()))(parser)
        ?? ((parser): Option<Node | Node[]> => node.decimal(parser.decimal()))(parser)
        ?? ((parser): Option<Node | Node[]> => funcall(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => typeinstancewithargs(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => ident(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => type(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => boolean(parser))(parser)
        ?? ((parser): Option<Node | Node[]> => none(parser))(parser)
        if (leafGroup) return getNodeArray(leafGroup)
        return null
    }
    const reservedAccepted: Set<string> = new Set(['true', 'false', 'none', 'and', 'or', 'not', 'if', 'else', 'for', 'while', 'loop', 'let', 'mut'])
    export function reserved (parser: TokenParser): Option<Node> {
        return node.string(parser.matchOneSet(reservedAccepted))
    }
    export function boolean (parser: TokenParser): Option<Node> {
        const booleanGroup = ((parser): Option<Node | Node[]> => node.string(parser.match('true')))(parser)
        ?? ((parser): Option<Node | Node[]> => node.string(parser.match('false')))(parser)
        if (booleanGroup) return {
            name: 'Boolean',
            items: getNodeArray(booleanGroup),
        }
        return null
    }
    export function none (parser: TokenParser): Option<Node> {
        const noneGroup = ((parser): Option<Node | Node[]> => node.string(parser.match('none')))(parser)
        if (noneGroup) return {
            name: 'None',
            items: getNodeArray(noneGroup),
        }
        return null
    }
    export function ident (parser: TokenParser): Option<Node[]> {
        const identGroup = parser.chain(
            parser => modifier.negative(parser, (parser): Option<Node | Node[]> => reserved(parser)),
            (parser): Option<Node | Node[]> => node.ident(parser.ident())
        )
        if (identGroup) return getNodeArray(identGroup)
        return null
    }
    export function parens (parser: TokenParser, inner: NodeParser): Option<Node[]> {
            const parensGroup = ((parser): Option<Node | Node[]> => bracketed(
                parser,
                (parser): Option<Node | Node[]> => inner(parser),
                (parser): Option<boolean> => parser.match('(') ? true : null,
                (parser): Option<boolean> => parser.match(')') ? true : null
        ))(parser)
        if (parensGroup) return getNodeArray(parensGroup)
        return null
    }
    export function parenlist (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            const parenlistGroup = ((parser): Option<Node | Node[]> => parens(
                parser,
                (parser): Option<Node | Node[]> => list(
                    parser,
                    (parser): Option<Node | Node[]> => inner(parser),
                    (parser): Option<Node | Node[]> => sep(parser)
                )
        ))(parser)
        if (parenlistGroup) return getNodeArray(parenlistGroup)
        return null
    }
    export function brackets (parser: TokenParser, inner: NodeParser): Option<Node[]> {
            const bracketsGroup = ((parser): Option<Node | Node[]> => bracketed(
                parser,
                (parser): Option<Node | Node[]> => inner(parser),
                (parser): Option<boolean> => parser.match('[') ? true : null,
                (parser): Option<boolean> => parser.match(']') ? true : null
        ))(parser)
        if (bracketsGroup) return getNodeArray(bracketsGroup)
        return null
    }
    export function bracketlist (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            const bracketlistGroup = ((parser): Option<Node | Node[]> => brackets(
                parser,
                (parser): Option<Node | Node[]> => list(
                    parser,
                    (parser): Option<Node | Node[]> => inner(parser),
                    (parser): Option<Node | Node[]> => sep(parser)
                )
        ))(parser)
        if (bracketlistGroup) return getNodeArray(bracketlistGroup)
        return null
    }
    export function braces (parser: TokenParser, inner: NodeParser): Option<Node[]> {
            const bracesGroup = ((parser): Option<Node | Node[]> => bracketed(
                parser,
                (parser): Option<Node | Node[]> => inner(parser),
                (parser): Option<boolean> => parser.match('{') ? true : null,
                (parser): Option<boolean> => parser.match('}') ? true : null
        ))(parser)
        if (bracesGroup) return getNodeArray(bracesGroup)
        return null
    }
    export function braceslist (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
            const braceslistGroup = ((parser): Option<Node | Node[]> => braces(
                parser,
                (parser): Option<Node | Node[]> => list(
                    parser,
                    (parser): Option<Node | Node[]> => inner(parser),
                    (parser): Option<Node | Node[]> => sep(parser)
                )
        ))(parser)
        if (braceslistGroup) return getNodeArray(braceslistGroup)
        return null
    }
    export function bracketed (parser: TokenParser, inner: NodeParser, open: NodeParser, close: NodeParser): Option<Node[]> {
        const bracketedGroup = parser.chain(
            (parser): Option<boolean> => open(parser) ? true : null,
            (parser): Option<Node | Node[]> => inner(parser),
            (parser): Option<boolean> => close(parser) ? true : null
        )
        if (bracketedGroup) return getNodeArray(bracketedGroup)
        return null
    }
    export function list (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
        const listGroup = parser.chain(
                parser => modifier.multiple(parser, (parser): Option<Node | Node[]> => listinner(
                    parser,
                    (parser): Option<Node | Node[]> => inner(parser),
                    (parser): Option<Node | Node[]> => sep(parser)
            )).flat(),
            parser => modifier.optional(parser, (parser): Option<Node | Node[]> => inner(parser))
        )
        if (listGroup) return getNodeArray(listGroup)
        return null
    }
    export function listinner (parser: TokenParser, inner: NodeParser, sep: NodeParser): Option<Node[]> {
        const listinnerGroup = parser.chain(
            (parser): Option<Node | Node[]> => inner(parser),
            (parser): Option<Node | Node[]> => sep(parser)
        )
        if (listinnerGroup) return getNodeArray(listinnerGroup)
        return null
    }
}