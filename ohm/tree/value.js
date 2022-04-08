function CupidValue (value) {
    return {
        value,

        resolve (scope) {
            return this
        },
        
        isEqual (newValue) {
            return value === newValue
        }
    }
}

function CupidInteger (value) { 
    return {
        ...CupidValue(value) 
    }
}

function CupidDecimal (value) {
    return {
        ...CupidValue(value) 
    }
}

function CupidString (value) {
    return {
        ...CupidValue(value) 
    }
}

function CupidBoolean (value) {
    return {
        ...CupidValue(value) 
    }
}

function CupidNone () {
    return {
        ...CupidValue(null) 
    }
}


export { CupidValue, CupidInteger, CupidDecimal, CupidString, CupidBoolean, CupidNone }