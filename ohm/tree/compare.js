import { CupidBoolean } from './index.js'

const Comparisons = {
    EQUAL: 0,
    NOT_EQUAL: 1,
    LESS_THAN: 2,
    LESS_OR_EQUAL: 3,
    GREATER_THAN: 4,
    GREATER_OR_EQUAL: 5,
    AND: 6,
    OR: 7,
}


function CupidCompare (operator, A, B) {
    return {
        operator,

        resolve (scope) {
            let a = A.resolve(scope).value
            let b = B.resolve(scope).value

            if ('values' in A && 'values' in B) {
                a = A.getValue()
                b = B.getValue()
            }

            switch (operator) {
                case Comparisons.EQUAL:
                    return CupidBoolean(a === b)
                case Comparisons.NOT_EQUAL:
                    return CupidBoolean(a !== b)
                case Comparisons.LESS_THAN:
                    return CupidBoolean(a < b)
                case Comparisons.LESS_OR_EQUAL:
                    return CupidBoolean(a <= b)
                case Comparisons.GREATER_THAN:
                    return CupidBoolean(a > b)
                case Comparisons.GREATER_OR_EQUAL:
                    return CupidBoolean(a >= b)
                case Comparisons.AND:
                    return CupidBoolean(a && b ? true : false)
                case Comparisons.OR:
                    return CupidBoolean(a || b ? true : false)
                default: CupidBoolean(false)
            }
        }
    }
}

export { Comparisons, CupidCompare }