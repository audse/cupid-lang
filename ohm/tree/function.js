import { CupidValue, CupidSymbol } from './index.js'

function CupidFunction (symbol, params, body) {

    return {
        symbol,
        params,
        body,

        resolve (scope) {
            return scope.setSymbol(CupidSymbol(symbol.name), function () {
                const innerScope = scope.makeSubScope()
                params.forEach((param, index) => innerScope.setSymbol(
                    CupidSymbol(param.name), arguments[index]
                ))
                return body.resolve(innerScope)
            })
        }
    }
}

function CupidAnonymousFunction (params, body) {
    return {
        params,
        body,

        resolve (scope) {
            return function () {
                const innerScope = scope.makeSubScope()
                params.forEach((param, index) => innerScope.setSymbol(
                    CupidSymbol(param.name), arguments[index]
                ))
                return body.resolve(innerScope)
            }
        }
    }
}

function CupidFunctionCall (fun, args) {

    return {
        fun,
        args,

        resolve (scope) {
            const funValue = scope.getSymbol(fun.name)
            return funValue.apply(null, args.resolve(scope))
        }
    }
}

function CupidAnonymousFunctionCall (symbol, args) {
    return {
        symbol,
        args,

        resolve (scope) {
            const funValue = symbol.resolve(scope)
            return funValue.apply(null, args.resolve(scope))
        }
    }
}

function CupidPropertyFunctionCall (dictionary, args) {
    return {
        ...CupidAnonymousFunctionCall(dictionary, args),
    }
}

function CupidArguments (args) {
    return {
        args,
        resolve (scope) {
            return args.map(arg => arg.resolve(scope))
        }
    }
}


export { CupidFunction, CupidAnonymousFunction, CupidPropertyFunctionCall, CupidFunctionCall, CupidArguments, CupidAnonymousFunctionCall }