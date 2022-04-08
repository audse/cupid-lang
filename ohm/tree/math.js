import { CupidValue } from './index.js'

const Operators = {
    ADD: 0,
    SUBTRACT: 1,
    MULTIPLY: 2,
    DIVIDE: 3,
    EXPONENT: 4,
    MOD: 5,
}

function CupidOperation (operator, A, B) {

    return {
        operator,
        
        resolve (scope) {
            const a = A.resolve(scope).value
            const b = B.resolve(scope).value
            
            return operator === Operators.ADD ? CupidValue(a + b)
            : operator === Operators.SUBTRACT ? CupidValue(a - b)
            : operator === Operators.MULTIPLY ? CupidValue(a * b)
            : operator === Operators.DIVIDE ? CupidValue(a / b)
            : operator === Operators.EXPONENT ? CupidValue(Math.pow(a, b))
            : operator === Operators.MOD ? CupidValue(a % b)
            : CupidValue(a)
        },
    }
}

export { Operators, CupidOperation }