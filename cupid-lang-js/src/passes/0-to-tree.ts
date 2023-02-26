import { Node, nodeIs, RuleNode } from '@/types'
import { safeStringify } from '@/utils'
import { isIdent, Kind, TypeKind } from '@/ast'
import { Expr, AnyTypeKind, Field } from './@types/1-pre-create-scope'
import * as make from './test/constructors'

type ToTree<Input extends Kind> = (
    (expr: Node, env: Node[], ...otherArgs: any[]) => Expr<Input>
)

type AcceptedExpr<K extends Kind> = {
    [Kind.Assign]: Kind.Assign
    [Kind.BinOp]: Kind
    [Kind.Block]: Kind.Block
    [Kind.Call]: Kind
    [Kind.Decl]: Kind.Decl
    [Kind.Fun]: Kind.Fun
    [Kind.Ident]: Kind.Ident
    [Kind.IfStmt]: Kind.IfStmt
    [Kind.Literal]: Kind.Literal
    [Kind.Map]: Kind.Map
    [Kind.Property]: Kind.Property
    [Kind.Type]: Kind.Type
    [Kind.TypeConstructor]: Kind.TypeConstructor
    [Kind.UnOp]: Kind
}[K]

type Methods = {
    [K in Kind]: ToTree<AcceptedExpr<K>>
}

function err (node: Node, message: string = 'Unexpected node') {
    console.trace()
    return `${ message }: ${ safeStringify(node) }`
}

function fieldToTree (node: Node, env: Node[]): Field {
    if (nodeIs.RuleNode(node)) {
        const [ident, type] = node.items
        return {
            ident: map.ident(ident, env),
            type: map.type(type, env),
        }
    }
    throw err(node)
}

const map: Methods = {

    [Kind.Assign]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            const [ident, value] = node.items
            return make.assign({
                ident: toTree<Kind.Ident>(ident, env),
                value: toTree(value, env)
            }, env.push(node))
        }
        throw err(node, 'Expected assign node')
    },

    [Kind.BinOp]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            if (node.items.length >= 3 && nodeIs.StringNode(node.items[1])) {
                if (['\\', '.'].includes(node.items[1].string)) return map.property(node, env)
                return make.binop({
                    left: toTree(node.items[0], env),
                    op: node.items[1].string,
                    right: toTree(node.items[2], env)
                }, env.push(node))
            }
            return toTree(node.items[0], env)
        }
        throw err(node)
    },

    [Kind.Block]: (node, env) => {
        if (nodeIs.RuleNode(node)) return make.block({
            exprs: node.items.map(item => toTree(item, env))
        }, env.push(node))
        throw err(node)
    },

    [Kind.Call]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            if (nodeIs.RuleNode(node.items[1])) return make.call({
                fun: toTree(node.items[0], env),
                args: node.items[1].items.map(item => toTree(item, env))
            }, env.push(node))
            return map.binop(node, env)
        }
        throw err(node)
    },

    [Kind.Decl]: (node, env, mutable: boolean = false) => {
        if (!nodeIs.RuleNode(node)) throw err(node)
        if (node.items.length === 3) {
            const [ident, typeHint, value] = node.items
            return make.decl({
                ident: map.ident(ident, env),
                type: map.type(typeHint, env),
                value: toTree(value, env)
            }, env.push(node))
        }
        const [ident, value] = node.items
        return make.decl({
            ident: map.ident(ident, env),
            type: make.unknown({}),
            value: toTree(value, env)
        }, env.push(node))
    },

    [Kind.Fun]: (node, env) => {
        if (nodeIs.RuleNode(node) && nodeIs.RuleNode(node.items[0])) {
            const [params, returnType, body] = (
                node.items.length >= 3 ? node.items
                    : [node.items[0], null, node.items[1]]
            )
            return make.fun({
                params: (params as RuleNode).items.map(field => fieldToTree(field, env)),
                body: toTree(body, env),
                returns: returnType ? map.type(returnType, env) : make.unknown({}),
            }, env.push(node))
        }
        throw err(node)
    },

    [Kind.Ident]: (node, env) => {
        if (nodeIs.IdentNode(node)) return make.ident({ name: node.ident }, env.push(node))
        throw err(node, 'Expected ident node')
    },

    [Kind.IfStmt]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            const [condition, body, elseBody] = node.items
            return make.ifstmt({
                condition: toTree(condition, env),
                body: toTree(body, env),
                elseBody: elseBody ? toTree(elseBody, env) : undefined
            })
        }
        throw err(node, 'expected rule node')
    },

    [Kind.Literal]: (node, env) => {
        if (nodeIs.StringNode(node)) return make.literal({ value: node.string }, env.push(node))
        if (nodeIs.DecimalNode(node)) return make.literal({ value: node.decimal }, env.push(node))
        if (nodeIs.IntNode(node)) return make.literal({ value: node.int }, env.push(node))
        if (nodeIs.RuleNode(node) && nodeIs.StringNode(node.items[0])) {
            switch (node.name) {
                case 'Boolean': return make.literal({ value: node.items[0].string === 'true' }, env.push(node.items[0]))
                case 'None': return make.literal({ value: null }, env.push(node.items[0]))
            }
        }
        throw err(node, 'Expected one of [string, decimal, int, boolean, none]')
    },

    [Kind.Map]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            const entries: [Expr<Kind.Literal>, Expr][] = node.items.map(item => {
                if (nodeIs.RuleNode(item)) {
                    const key = toTree(item.items[0], env)
                    const value = toTree(item.items[1], env)
                    if (isIdent<Expr<Kind.Ident>>(key)) return [
                        make.literal({ value: key.name }, key.source),
                        value
                    ]
                    return [key as Expr<Kind.Literal>, value]
                }
                throw err(item, 'expected rule node')
            })
            return make.map({ entries }, env.push(node))
        }
        throw err(node, 'expected rule node')
    },

    [Kind.Property]: (node, env) => {
        const [left, _, rightNode] = (node as RuleNode).items

        const right = toTree(rightNode, env)
        // convert ident properties to string properties
        const property = (
            isIdent<Expr<Kind.Ident>>(right) ? make.literal({ value: right.name }, right.source)
                : right
        )

        return make.property({ parent: toTree(left, env), property }, env.push(node))
    },

    [Kind.Type]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            if (node.name === 'StructType') return make.struct({
                fields: node.items.map(item => fieldToTree(item, env))
            }, env.push(node))

            if (node.name === 'SumType') return make.sum({
                fields: node.items.map(item => fieldToTree(item, env))
            }, env.push(node))

            if (node.name === 'PrimitiveType') return make.primitive({
                name: map.ident(node.items[0], env).name
            }, env.push(node))
            if (node.name === 'TypeHint') return map.type(node.items[0], env)
            if (['TypeInstance', 'TypeInstanceWithArgs'].includes(node.name)) {
                const [ident, args] = node.items
                return make.typeInstance({
                    ident: map.ident(ident, env),
                    args: args && nodeIs.RuleNode(args) ? args.items.map(arg => map.type(arg, env)) : []
                }, env.push(node))
            }
        }
        if (nodeIs.IdentNode(node)) return make.typeInstance({
            ident: map.ident(node, env),
            args: []
        }, env.push(node))

        throw err(node, 'Expected type node')
    },

    [Kind.TypeConstructor]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            if (node.items.length >= 3) {
                const [identNode, paramsNode, typeNode] = node.items
                const ident = map.ident(identNode, env)
                if (nodeIs.RuleNode(paramsNode)) {
                    const params = paramsNode.items.map(item => map.ident(item, env))
                    const value = map.type(typeNode, env)
                    return make.typeConstructor({ ident, params, value }, env.push(node))
                }
            }
            const [identNode, typeNode] = node.items
            const ident = map.ident(identNode, env)
            const value = map.type(typeNode, env)
            return make.typeConstructor({ ident, params: [], value }, env.push(node))
        }
        throw err(node, 'Expected type constructor node')
    },

    [Kind.UnOp]: (node, env) => {
        if (nodeIs.RuleNode(node)) {
            if (node.items.length >= 2 && nodeIs.StringNode(node.items[0])) return make.unop({
                op: node.items[0].string,
                expr: toTree(node.items[1], env)
            }, env.push(node))
            return toTree(node.items[0], env) as Expr<Kind.UnOp>
        }
        throw err(node)
    },
}

export function toTree<
    Input extends Kind = Kind
> (node: Node, env: Node[]): Expr<Input> {
    if (nodeIs.RuleNode(node)) {
        switch (node.name) {
            case 'Assign': return map.assign(node, env) as Expr<Input>
            case 'BinaryOp': return map.binop(node, env) as Expr<Input>
            case 'Block': return map.block(node, env) as Expr<Input>
            case 'FunCall': return map.call(node, env) as Expr<Input>
            case 'Declare': return map.decl(node, env) as Expr<Input>
            case 'DeclareMut': return map.decl(node, env) as Expr<Input>
            case 'Func': return map.fun(node, env) as Expr<Input>
            case 'IfStmt': return map.ifstmt(node, env) as Expr<Input>
            case 'MapLiteral': return map.map(node, env) as Expr<Input>
            case 'TypeConstructor': return map.typedef(node, env) as Expr<Input>
            case 'UnaryOp': return map.unop(node, env) as Expr<Input>

            case 'PrimitiveType':
            case 'StructType':
            case 'TypeInstance':
            case 'TypeInstanceWithArgs':
            case 'SumType': return map.type(node, env) as Expr<Input>
            case 'TypeHint': return toTree(node.items[0], env)
        }
    }
    if (nodeIs.IdentNode(node)) return map.ident(node, env) as Expr<Input>
    return map.literal(node, env) as Expr<Input>
}