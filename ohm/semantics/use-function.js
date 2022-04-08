import {
    CupidFunction,
    CupidAnonymousFunction,
    CupidFunctionCall,
    CupidAnonymousFunctionCall,
    CupidPropertyFunctionCall,
    CupidArguments
} from './../tree/index.js'


const useFunction = {
    BaseFunction: (_, name, __, params, ___, block) => CupidFunction(
        name.toTree(), params.toTree(), block.toTree()
    ),

    AnonymousFunction: (params, _, block) => CupidAnonymousFunction(
        params.toTree(), block.toTree()
    ),

    Parameters: a => a.asIteration().toTree(),

    FunctionCall_base: (funName, args) => CupidFunctionCall(
        funName.toTree(), args.toTree()
    ),

    FunctionCall_dictionary_property: (dictFun, args) => CupidPropertyFunctionCall(
        dictFun.toTree(), args.toTree()
    ),

    FunctionCall_inline_anonymous: (_, args, __, fun, ___) => CupidAnonymousFunctionCall(
        fun.toTree(), args.toTree()
    ),

    BaseFunctionCall: (_, args, __) => args.toTree(),

    Arguments: a => CupidArguments(a.asIteration().toTree()),
}

export { useFunction }