import {
    CupidWhileLoop,
    CupidForInLoop,
} from './../tree/index.js'

const useLoop = {
    WhileLoop: (_, condition, body) => CupidWhileLoop(
        condition.toTree(), body.toTree()
    ),

    ForInLoop: (_, identifiers, __, iterable, body) => CupidForInLoop(
        identifiers.toTree(), iterable.toTree(), body.toTree()
    ),

    ForInIdentifiers_parentheses: (_, identifiers, __) => identifiers.asIteration().toTree(),
    ForInIdentifiers_no_parentheses: identifiers => identifiers.asIteration().toTree(),
}

export { useLoop }