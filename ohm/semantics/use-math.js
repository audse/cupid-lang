import {
    CupidOperation,
    Operators,
} from './../tree/index.js'

const useMath = {
    Addition_addition: (a, _, b) => CupidOperation(
        Operators.ADD, a.toTree(), b.toTree()
    ),
    Addition_subtraction: (a, _, b) => CupidOperation(
            Operators.SUBTRACT, a.toTree(), b.toTree()
    ),
    Multiplication_multiplication: (a, _, b) => CupidOperation(
            Operators.MULTIPLY, a.toTree(), b.toTree()
    ), 
    Multiplication_division: (a, _, b) => CupidOperation(
            Operators.DIVIDE, a.toTree(), b.toTree()
    ), 
    Multiplication_modulus: (a, _, b) => CupidOperation(
            Operators.MOD, a.toTree(), b.toTree()
    ), 
    Exponent_exponent: (a, _, b) => CupidOperation(
            Operators.EXPONENT, a.toTree(), b.toTree()
    ),
    Primary_parentheses: (_, a, __) => a.toTree(),
    Primary_negative: (_, a) => -a.toTree(),
}

export { useMath }