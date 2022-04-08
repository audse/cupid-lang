function CupidBlock (block) {
    return {
        block,

        resolve (scope) {
            const values = block.map(expression => expression.resolve(scope))
            return values.pop() // return last statement's value
        }
    }
}


function CupidIfBlock (condition, ifBody, elseIfStatements, elseBody) {
    return {
        condition,
        ifBody,
        elseBody,

        resolve (scope) {
            const value = condition.resolve(scope)
            if (value.value) return ifBody.resolve(scope)
            else if (elseIfStatements.length) {
                elseIfStatements.forEach(elseIfStatement => {
                    const [condition, block] = elseIfStatement
                    const value = condition.resolve(scope)
                    if (value.value) return block.resolve(scope)
                })
            } else if (elseBody) return elseBody.resolve(scope)
        }
    }
}

export { CupidBlock, CupidIfBlock }