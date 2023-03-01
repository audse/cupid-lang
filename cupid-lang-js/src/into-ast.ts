import { CustomNode, DecimalNode, IdentNode, IntNode, Node, nodeIs, RuleNode, StringNode } from '@/types'
import { Assign, BinOp, Block, Call, Decl, Environment, Expr, FieldType, Fun, FunType, Ident, Impl, InstanceType, Literal, Lookup, PrimitiveType, StructType, Type, TypeConstructor, UnOp } from '@/ast'
import { Scope } from './env'

export function intoAst () {

    const scope = new Scope()
    const source: Node[] = []

    function into (node: Node): Expr {
        if (nodeIs.RuleNode(node)) {
            switch (node.name) {
                case 'Assign': return intoAssign(node)
                case 'BinOp': return intoBinOp(node)
                case 'Block': return intoBlock(node)
                case 'Bool': return intoBool(node)
                case 'Call': return intoCall(node)
                case 'Decl': return intoDecl(node)
                case 'DeclMut': return intoDecl(node, true)
                case 'Fun': return intoFun(node)
                case 'Impl': return intoImpl(node)
                case 'Lookup': return intoLookup(node)
                case 'None': return intoNone(node)
                case 'TypeConstructor': return intoTypeConstructor(node)
                case 'UnOp': return intoUnOp(node)

                /** Types */
                case 'InstanceType':
                case 'InstanceTypeWithArgs':
                case 'PrimitiveType':
                case 'StructType':
                case 'TypeHint': return intoType(node)
            }
        }
        if (nodeIs.IdentNode(node)) return intoIdent(node)
        if (nodeIs.DecimalNode(node)) return intoDec(node)
        if (nodeIs.StringNode(node)) return intoString(node)
        if (nodeIs.IntNode(node)) return intoInt(node)
        throw `unimplemented node: ${ JSON.stringify(node, null, 2) }`
    }

    function err (expected: string, found: Node) {
        if (!found) console.trace()
        console.error(`\nexpected ${ expected } node, found ${ JSON.stringify(found, null, 2) }`)
        return found
    }

    function rule<T, Args = undefined> (predicate: (node: RuleNode, args?: Args) => T): (node: Node, args?: Args) => T {
        return (node, args) => {
            if (nodeIs.RuleNode(node)) return predicate(node, args)
            throw err('rule', node)
        }
    }

    function ident<T> (predicate: (node: IdentNode) => T): (node: Node) => T {
        return node => {
            if (nodeIs.IdentNode(node)) return predicate(node)
            throw err('ident', node)
        }
    }

    function string<T> (predicate: (node: StringNode) => T): (node: Node) => T {
        return node => {
            if (nodeIs.StringNode(node)) return predicate(node)
            throw err('string', node)
        }
    }

    function int<T> (predicate: (node: IntNode) => T): (node: Node) => T {
        return node => {
            if (nodeIs.IntNode(node)) return predicate(node)
            throw err('int', node)
        }
    }

    function dec<T> (predicate: (node: DecimalNode) => T): (node: Node) => T {
        return node => {
            if (nodeIs.DecimalNode(node)) return predicate(node)
            throw err('decimal', node)
        }
    }

    const intoAssign = rule(node => {
        const [ident, value] = node.items
        return new Assign({
            scope,
            source: source.push(node),
            ident: intoIdent(ident),
            value: into(value)
        })
    })

    const intoBinOp = rule(node => {
        if (node.items.length > 1) {
            const [left, op, right] = node.items
            return new BinOp({
                scope,
                source: source.push(node),
                left: into(left),
                right: into(right),
                op: string(str => str.string)(op)
            })
        }
        return into(node.items[0])
    })

    const intoBlock = rule(node => new Block({
        scope,
        source: source.push(node),
        exprs: node.items.map(into)
    }))

    const intoBool = rule(node => new Literal({
        scope,
        source: source.push(node),
        value: string(node => node.string)(node.items[0]) === 'true'
    }))

    const intoCall = rule(node => {
        if (node.items.length > 1) {
            const [fun, args] = node.items
            return new Call({
                scope,
                source: source.push(node),
                fun: into(fun),
                args: intoCallArgs(args)
            })
        }
        return into(node.items[0])
    })

    const intoCallArgs = rule(node => {
        return node.items.map(arg => into(arg))
    })

    const intoDecl = rule<Decl, boolean>((node, mutable) => {
        const [ident, type, value] = node.items.length === 3 ? node.items : [node.items[0], null, node.items[1]]
        return new Decl({
            scope,
            mutable,
            source: source.push(node),
            ident: intoIdent(ident),
            value: into(value),
            ...type && { type: intoType(type) },
        })
    })

    const intoFun = rule(node => {
        const [paramsNode, returns, body] = node.items.length === 3 ? node.items : [node.items[0], null, node.items[1]]
        const { hasSelfParam, params } = intoParams(paramsNode)
        return new Fun({
            scope,
            params,
            hasSelfParam,
            source: source.push(node),
            returns: returns ? intoType(returns) : undefined,
            body: into(body),
        })
    })

    const intoParams = rule<{ hasSelfParam: boolean, params: FieldType[] }>(node => {
        const selfParam = node.items.findIndex(param => rule(p => nodeIs.StringNode(p.items[0]) && p.items[0].string === 'self')(param))
        const params = node.items.filter((_, i) => i !== selfParam).map(item => intoFieldType(item))
        return { hasSelfParam: selfParam !== -1, params }
    })

    const intoIdent = ident(node => new Ident({
        scope,
        source: source.push(node),
        name: node.ident
    }))

    const intoImpl = rule(node => {
        const [type, ...funs] = node.items
        return new Impl({
            scope,
            source: source.push(node),
            type: intoType(type),
            environment: intoImplEnvironment(funs)
        })
    })

    const intoImplEnvironment = (nodes: Node[]) => new Environment({
        scope,
        content: nodes.map(node => intoImplFunDecl(node))
    })

    const intoImplFunDecl = rule(node => {
        const [ident, fun] = node.items
        return new Decl({
            scope,
            source: source.push(node),
            ident: intoIdent(ident),
            value: intoFun(fun)
        })
    })

    const intoLookup = rule(node => {
        if (node.items.length > 2) {
            const [env, _, memberNode] = node.items
            const member = into(memberNode)
            if (member instanceof Ident || member instanceof Literal) return new Lookup({
                scope,
                source: source.push(node),
                environment: into(env),
                member,
            })
            throw `environments can only be accessed by idents or literals`
        }
        return into(node.items[0])
    })

    const intoNone = (node: Node) => new Literal({
        scope,
        source: source.push(node),
        value: null
    })

    const intoTypeConstructor = rule(node => {
        const [identNode, params, value] = node.items.length === 3 ? node.items : [node.items[0], null, node.items[1]]
        const ident = (
            nodeIs.IdentNode(identNode) ? intoIdent(identNode)
                : string(ident => intoIdent({ ...ident, ident: ident.string }))(identNode)
        )
        return new TypeConstructor({
            scope,
            ident,
            source: source.push(node),
            body: intoType(value),
            params: params ? rule(p => p.items.map(item => intoIdent(item)))(params) : []
        })
    })

    const intoUnOp = rule(node => {
        if (node.items.length === 2) return new UnOp({
            scope,
            source: source.push(node),
            expr: into(node.items[1]),
            op: string(op => op.string)(node.items[0])
        })
        return into(node.items[0])
    })

    /* Types */

    const intoType: (node: Node) => Type = rule<Type>(node => {
        switch (node.name) {
            case 'FieldType': return intoFieldType(node)
            case 'InstanceType':
            case 'InstanceTypeWithArgs': return intoInstanceType(node)
            case 'PrimitiveType': return intoPrimitiveType(node)
            case 'StructType': return intoStructType(node)
            case 'TypeHint': return intoType(node.items[0])
        }
        throw `unexpected type node: ${ node.name }`
    })

    const intoFieldType = rule<FieldType>(node => {
        const [ident, type] = node.items
        return new FieldType({
            scope,
            source: source.push(node),
            ident: intoIdent(ident),
            type: intoType(type)
        })
    })

    const intoInstanceType = rule<InstanceType>(node => {
        const [ident, args] = node.items
        return new InstanceType({
            scope,
            source: source.push(node),
            ident: intoIdent(ident),
            args: args ? intoInstanceTypeArgs(args) : []
        })
    })

    const intoInstanceTypeArgs = rule<Type[]>(node => node.items.map(item => intoType(item)))

    const intoPrimitiveType = rule<PrimitiveType>(node => new PrimitiveType({
        scope,
        source: source.push(node),
        name: string(node => node.string)(node.items[0]).replace(/'"`/g, '')
    }))

    const intoStructType = rule<StructType>(node => new StructType({
        scope,
        source: source.push(node),
        fields: node.items.map(item => intoFieldType(item)),
    }))

    const intoString = string(node => new Literal({
        scope,
        source: source.push(node),
        value: node.string
    }))

    const intoInt = int(node => new Literal({
        scope,
        source: source.push(node),
        value: node.int
    }))

    const intoDec = dec(node => new Literal({
        scope,
        source: source.push(node) - 1,
        value: node.decimal
    }))


    return { into, scope, source }
}
