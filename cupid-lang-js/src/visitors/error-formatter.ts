import { Expr, ExprVisitor, BinOp, Ident, Literal, FunType, PrimitiveType, StructType, Type, TypeConstructor, FieldType, UnknownType, Decl, Assign, Block, InstanceType, Lookup, Environment, Impl, ExprVisitorWithContext, Call } from '@/ast'
import { CompilationError } from '@/error/compilation-error'
import { FileFormatter } from '@/fmt/utils'
import { Node, nodeIs } from '@/types'
import { BaseExprVisitorWithContext } from './base'

export default class ErrorFormatter extends BaseExprVisitorWithContext<{ fmt: FileFormatter, source: Node[] }> {

    visitIdent (ident: Ident, context: { fmt: FileFormatter; source: Node[] }): void {
        const node = context.source[ident.source - 1]
        if (nodeIs.IdentNode(node)) context.fmt.underlineToken(node.token)
    }

    visitLiteral (literal: Literal, context: { fmt: FileFormatter; source: Node[] }): void {
        const node = context.source[literal.source - 1]
        if (!nodeIs.RuleNode(node)) context.fmt.underlineToken(node.token)
    }

    /** Types */

    visitPrimitiveType (primitive: PrimitiveType, context: { fmt: FileFormatter; source: Node[] }): void {
        const node = context.source[primitive.source - 1]
        if (!nodeIs.RuleNode(node)) context.fmt.underlineToken(node.token)
    }

}