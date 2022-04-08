import {
    CupidAssignment,
    CupidDeclaration,
    CupidConstantDeclaration,
    CupidSymbol,
    CupidPropertyAssignment,
    CupidOperation,
    Operators,
} from './../tree/index.js'

const useAssignment = {
    BaseAssignment: (a, _, b) => CupidAssignment(a.toTree(), b.toTree()),

    DeclarationAssignment_constant: (_, a, __, b) => CupidConstantDeclaration(a.toTree(), b.toTree()),
    DeclarationAssignment_variable: (_, a, __, b) => CupidDeclaration(a.toTree(), b.toTree()),

    PropertyAssignment: (propertyAccess, _, value) => CupidPropertyAssignment(propertyAccess.toTree(), value.toTree()),

    OperatorAssignment_addition: (a, _, b) => CupidAssignment(
        a.toTree(), 
        CupidOperation(Operators.ADD, a.toTree(), b.toTree())
    ),
    OperatorAssignment_subtraction: (a, _, b) => CupidOperation(
            Operators.SUBTRACT, a.toTree(), b.toTree()
    ),
    OperatorAssignment_multiplication: (a, _, b) => CupidOperation(
            Operators.MULTIPLY, a.toTree(), b.toTree()
    ), 
    OperatorAssignment_division: (a, _, b) => CupidOperation(
            Operators.DIVIDE, a.toTree(), b.toTree()
    ), 
    OperatorAssignment_modulus: (a, _, b) => CupidOperation(
            Operators.MOD, a.toTree(), b.toTree()
    ), 
    OperatorAssignment_exponent: (a, _, b) => CupidOperation(
            Operators.EXPONENT, a.toTree(), b.toTree()
    ),

    base_identifier (a, b) {
        return CupidSymbol(this.sourceString)
    },

    constant_identifier (a) {
        return CupidSymbol(this.sourceString)
    }
}

export { useAssignment }