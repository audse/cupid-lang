import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Lookup, Environment } from '@/ast'
import { CompilationError } from '@/error/compilation-error'
import BaseExprVisitor from './base'


/**
 * Annotate all idents with references to their associated symbol & checks immutability
 */
export default class SymbolResolver extends BaseExprVisitor {

    visitAssign (assign: Assign): void {
        super.visitAssign(assign)
        const symbol = assign.ident.expectSymbol()
        if (!symbol.mutable) throw CompilationError.immutable(assign.ident)
    }

    visitIdent (ident: Ident): void {
        ident.symbol = ident.scope.lookupExpect(ident)

        if (ident.symbol.value instanceof Environment) {
            ident.scope = ident.symbol.value.scope
        }
    }

    visitLookup (lookup: Lookup): void {
        lookup.environment.accept(this)
        // Skip member visit for now, must be resolved after types are resolved
    }

}