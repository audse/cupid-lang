import { 
    CupidValue,
    CupidSymbol,
} from './index.js'

function CupidWhileLoop (condition, body) {

    return {
        condition,
        body,

        resolve (scope) {
            let returnValue = CupidValue(null)
            while (true) {
                const conditionValue = condition.resolve(scope)
                if (conditionValue.isEqual(false)) break
                returnValue = body.resolve(scope)
            }
            return returnValue
        }
    }
}

function CupidForLoop () {}

function CupidForInLoop (identifiers, iterable, body) {
    return {
        identifiers,
        iterable,
        body,

        resolve (scope) {            
            let returnValue = CupidValue(null)

            const iterableValue = iterable.resolve(scope).toIterable(scope)

            let index = 0
            for (const [key, value] of iterableValue.entries()) {
                const innerScope = scope.makeSubScope()
                if (identifiers.length === 1) { // get value
                    innerScope.setSymbol(identifiers[0], value)

                } else if (identifiers.length === 2) { // get key, value
                    innerScope.setSymbol(identifiers[0], key)
                    innerScope.setSymbol(identifiers[1], value)

                } else if (identifiers.length === 3) { // get index, key, value
                    innerScope.setSymbol(identifiers[0], CupidValue(index))
                    innerScope.setSymbol(identifiers[1], key)
                    innerScope.setSymbol(identifiers[2], value)
                }
                returnValue = body.resolve(innerScope)
                index++
            }
            return returnValue
        }
    }
}


export { CupidWhileLoop, CupidForLoop, CupidForInLoop }