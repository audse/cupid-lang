import {
    CupidList,
    CupidDictionary,
    CupidTuple,
    CupidRange,
    CupidPropertyAccess,
} from './../tree/index.js'

const useStructures = {
    List: (_, values, __) => CupidList(values.asIteration().toTree()),

    Dictionary: (_, values, __) => CupidDictionary(values.asIteration().toTree()),
    DictionaryElement: (key, _, value) => [key.toTree(), value.toTree()],

    PropertyAccess_index: (object, _, index) => CupidPropertyAccess(object.toTree(), index.toTree()),
    PropertyAccess_identifier: (object, _, identifier) => CupidPropertyAccess(object.toTree(), identifier.toTree()),
    PropertyAccess_term: (object, _, term, __) => CupidPropertyAccess(object.toTree(), term.toTree()),

    Tuple: (_, values, __) => CupidTuple(values.asIteration().toTree()),

    Range_exclusive: (start, _, end) => CupidRange(start.toTree(), end.toTree(), false, false),
    Range_inclusive: (_, start, __, end, ___) => CupidRange(start.toTree(), end.toTree(), true, true),
    Range_exclusive_start_inclusive_end: (start, _, end, __) => CupidRange(start.toTree(), end.toTree(), false, true),
    Range_inclusive_start_exclusive_end: (_, start, __, end) => CupidRange(start.toTree(), end.toTree(), true, false),
}

export { useStructures }