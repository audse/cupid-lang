import { 
    CupidNone,
    CupidValue,
} from './index.js'

function CupidSymbol (name, mutable=true) {
    return {
        name,
        mutable,

        resolve (scope) {
            return scope.getSymbol(name)
        },
    }
}

function CupidStorage (key, value, mutable=true) {
    return {
        key,
        mutable,
        set value (newValue) {
            if (mutable) {
                value = newValue
            } else {
                console.log('Attempted to write to a constant symbol', key)
                // throw new Error(`${ key } is not mutable.`)
                // fail silently
            }
            return value
        },
        get value () { return value }
    }
}


function CupidScope (parent) {

    const storage = new Map()

    function getSymbol (name) {
        if (storage.has(name)) return storage.get(name).value
        return getParentSymbol(name)
    }

    function setSymbol (symbol, object) {
        const storageValue = storage.get(symbol.name)
        const parentValue = getParentSymbol(symbol.name)

        if (storageValue) {
            storageValue.value = object
        } else if (parentValue) {
            setParentSymbol(symbol, object)
        } else {
            storage.set(symbol.name, CupidStorage(symbol.name, object, symbol.mutable))
        }

        return getSymbol(symbol.name)
    }

    function setParentSymbol (symbol, object) {
        if (parent) return parent.setSymbol(symbol, object)
        return null
    }

    function getParentSymbol (name) {
        if (parent) return parent.getSymbol(name)
        else return null
    }

    function makeSubScope () {
        return CupidScope(this)
    }

    function toIterable () {
        var iterable = Array.from(storage, ([key, value]) => [CupidValue(key), value.value])
        return new Map(iterable)
    }

    return {
        storage,
        parent,
        getSymbol,
        setSymbol,
        setParentSymbol,
        makeSubScope,
        toIterable,
    }
}

export { CupidSymbol, CupidScope }