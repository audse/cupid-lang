import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Lookup, Environment, Impl } from '@/ast'
import { CompilationError } from '@/error/compilation-error'
import BaseExprVisitor from './base'

/**
 * Attempts to locate all environment lookup references
 */
export default class LookupMemberResolver extends BaseExprVisitor {

    visitLookup (lookup: Lookup): void {
        lookup.environment.accept(this)

        lookup.member = (() => {
            if (lookup.member instanceof Ident) return lookup.member
            if (lookup.member instanceof Literal) return lookup.member.intoIdent()
            throw CompilationError.unableToResolveLookup(lookup)
        })()

        lookup.member.symbol = (() => {
            for (const scope of lookup.environment.lookupEnvironments) {
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