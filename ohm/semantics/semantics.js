import {
    useAssignment,
    useBlock,
    useCompare,
    useFunction,
    useLoop,
    useMath,
    useValue,
    useStructures,
} from './index.js'

function CupidSemantics (semantics) {

    const makeTree = semantics.addOperation('toTree', {
        ...useValue,
        ...useMath,
        ...useCompare,
        ...useBlock,
        ...useLoop,
        ...useFunction,
        ...useAssignment,
        ...useStructures,
    })

    return {
        makeTree
    }
}

export { CupidSemantics }