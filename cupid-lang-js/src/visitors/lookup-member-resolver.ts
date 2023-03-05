import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Lookup, Environment, Impl } from '@/ast'
import { CompilationError } from '@/error/compilation-error'
import BaseExprVisitor from './base'

/**
 * Attempts to locate all environment lookup references
 */
export default class LookupMemberResolver extends BaseExprVisitor {

    visitLookup (lookup: Lookup): void {
        lookup.environment.accept(this)

        let env = lookup.environment

        if (lookup.environment instanceof Ident) {
            const symbol = lookup.environment.expectSymbol()
            if (symbol.value) env = symbol.value
        }

        if (env instanceof Lookup) {
            const symbol = (env.member as Ident).expectSymbol()
            if (symbol.value && symbol.value instanceof Environment) lookup.lookupEnvironments = [symbol.value.scope]
        }

        lookup.member = (() => {
            if (lookup.member instanceof Ident) return lookup.member
            if (lookup.member instanceof Literal) return lookup.member.intoIdent()
            throw CompilationError.unableToResolveLookup(lookup)
        })()

        lookup.member.symbol = (() => {
            for (const scope of lookup.lookupEnvironments) {
                const symbol = scope.lookup(lookup.member)
                if (symbol) {
                    lookup.member.inferredType = symbol.value?.inferredType || symbol.type
                    lookup.inferredType = lookup.member.inferredType
                    return symbol
                }
            }
            throw CompilationError.notDefined(lookup.member)
        })()

    }

}