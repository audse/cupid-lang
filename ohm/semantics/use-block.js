import {
    CupidBlock,
    CupidIfBlock,
} from './../tree/index.js'

const useBlock = {

    BraceBlock: (_, body, __) => CupidBlock(body.toTree()),
    ArrowBlock: (_, a) => a.toTree(),

    IfBlock: (_, condition, ifBlock, elseIfBlocks, ___, elseBlock) => {
        const ifBody = ifBlock.toTree()
        const elseBody = elseBlock ? elseBlock.toTree()[0] : null
        return CupidIfBlock(condition.toTree(), ifBody, elseIfBlocks.toTree(), elseBody)
    },

    ElseIfBlock: (_, condition, block) => {
        return [condition.toTree(), block.toTree()]
    }
}

export { useBlock }