
function CupidAssignment (symbol, value) {
    
    return {
        symbol,
        value,

        resolve (scope) {
            return scope.setSymbol(symbol, value.resolve(scope))
        }
    }
}

function CupidDeclaration (symbol, value) {
    return {
        symbol,
        value,

        resolve (scope) {
            return scope.setSymbol(symbol, value.resolve(scope))
        }
    }
}

function CupidConstantDeclaration (symbol, value) {
    symbol.mutable = false
    return {
        symbol,
        value,
        resolve (scope) {
            return scope.setSymbol(symbol, value.resolve(scope))
        }
    }
}

function CupidPropertyAssignment (property, value) {

    return {
        property,
        value,

        resolve (scope) {
            const propertyValue = property.resolve(scope, true)
            return propertyValue.objectValue.set(propertyValue.termValue, value.resolve(scope))
        }
    }
}

export { CupidAssignment, CupidDeclaration, CupidConstantDeclaration, CupidPropertyAssignment }