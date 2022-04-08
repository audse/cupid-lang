import {
    CupidDecimal,
    CupidInteger,
    CupidString,
    CupidBoolean,
    CupidNone,
} from './../tree/index.js'

const useValue = {
    true: a => true,
    false: a => false,
    none: a => null,
    constant_null: a => CupidNone(),
    constant_boolean: a => CupidBoolean(a.toTree()),

    constant_number: a => a.toTree(),
    constant_decimal (a, b, c) { return CupidDecimal(parseFloat(this.sourceString)) },
    constant_big_int (a, b, c, d, e, f, g, h) {
        const int = this.sourceString.replace(/\_/g, '')
        return CupidInteger(parseInt(int))
    },

    String: (_, a, __) => CupidString(a.sourceString),
    StringTemplate: (_, a, __) => a.toTree(),

    Group: (_, a, __) => a.toTree(),
}

export { useValue }