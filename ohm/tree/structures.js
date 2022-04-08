import { 
    CupidSymbol,
    CupidNone,
    CupidInteger
} from './index.js'


function CupidDictionary (values) {

    function getSymbol (key) {
        return 'name' in key ? key : CupidSymbol(key.value)
    }

    return {
        values,
        getSymbol,
        innerScope: null,

        resolve (scope) {
            this.innerScope = scope.makeSubScope()
            values.forEach(
                ([key, value]) => this.innerScope.setSymbol(getSymbol(key, scope), value)
            )
            return this
        },

        isEqual (newValues) {
            return Object.keys(newValues).every(key => {
                const symbol = this.innerScope.getSymbol(key)
                if (symbol) return symbol.resolve(this.innerScope).isEqual(newValues[key])
            })
        },

        get (key) {
            return this.innerScope.getSymbol(getSymbol(key).name) || CupidNone()
        },

        set (key, value) {
            values[key] = value
            this.innerScope.setSymbol(key, value)
            return this
        },
        
        getValue () {
            const valueDict = {}
            values.forEach(([key, val]) => {
                valueDict[key.name] = val.resolve(this.innerScope).value
            })
            return valueDict
        },

        toIterable (scope) {
            return this.innerScope.toIterable()
        },

        get size () {
            return this.innerScope.toIterable().size
        }
    }
}

function CupidList (listValues) {
    const {
        getSymbol,
        innerScope,
        resolve,
        get,
        set,
        toIterable,
    } = CupidDictionary(listValues.map((value, index) => [CupidSymbol(index), value]))

    return {
        values: listValues,
        innerScope,
        resolve,

        isEqual (newValues) {
            return newValues.every((value, index) => {
                let symbol = this.innerScope.getSymbol(index)
                if (symbol) {
                    symbol = symbol.resolve(this.innerScope)
                    return symbol.isEqual(value)
                }
            })
        },
        get,
        set,

        getValue () {
            const valueList = []
            listValues.forEach((val, index) => {
                const value = val.resolve(this.innerScope)
                if ('value' in value) valueList[index] = value.value
                else if ('getValue' in value) valueList[index] = value.getValue()
                else valueList[index] = value
            })
            return valueList
        },

        getSymbol,
        toIterable,
    }
}


function CupidTuple (tupleValues) {
    return {
        ...CupidList(tupleValues)
    }
}

function CupidRange (start, end, startInclusive, endInclusive) {
    const {
        values,
        innerScope,
        get,
        set,
        getSymbol,
        toIterable,
    } = CupidList([])

    function getList (scope) {
        let startNum = start.resolve(scope).value + (!startInclusive ? 1 : 0)
        let endNum = end.resolve(scope).value + (endInclusive ? 1 : 0)
        return Array.from({ length: endNum - startNum }, (_, index) => index + startNum)
    }

    return {
        start,
        end,
        startInclusive,
        endInclusive,
        innerScope,
        values,

        resolve (scope) {
            this.innerScope = scope.makeSubScope()
            this.values = getList(scope)
            this.values.forEach(index => this.innerScope.setSymbol(CupidSymbol(index), CupidInteger(index)))
            return this
        },

        getValue () {
            const valueList = []
            this.values.forEach((val, index) => {
                valueList[index] = val.value
            })
            return valueList
        },

        isEqual (newValues) {
            return newValues.every((value, index) => value === this.values[index])
        },
        
        get,
        set,
        
        getSymbol,
        toIterable,
    }
}


function CupidPropertyAccess (object, term) {

    function getTermValue (resolvedObject, blockScope) {
        const blockScopedTerm = term.resolve(blockScope)
        const objectScopedTerm = term.resolve(resolvedObject.innerScope)

        if (objectScopedTerm !== null && blockScopedTerm === null) {
            return term // term only exists inside object, can't be gotten from outer scope
        } else if (objectScopedTerm !== null) {
            return objectScopedTerm // term exists inside object and outer scope, prioritize object scope
        } else if (blockScopedTerm !== null) {
            return blockScopedTerm // term exists only in outer scope, prioritize outer scope
        } else {
            return CupidNone() // term doesn't exist
        }
    }

    function getObjectValue (scope) {
        let objectValue = object
        while (!objectValue.innerScope) {
            objectValue = objectValue.resolve(scope)
        }
        return objectValue
    }

    return {
        object,
        objectValue: null,
        term,
        termValue: null,

        resolve (scope, getBody) {
            this.objectValue = getObjectValue(scope)
            this.termValue = getTermValue(this.objectValue, scope)

            if (!getBody) return this.objectValue.get(this.termValue).resolve(scope)
            else return this
        }
    }
}

export { CupidDictionary, CupidList, CupidTuple, CupidRange, CupidPropertyAccess }