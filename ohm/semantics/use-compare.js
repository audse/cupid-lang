import {
    CupidCompare,
    Comparisons,
} from './../tree/index.js'

const useCompare = {
    Comparison_equal: (a, _, b) => CupidCompare(
        Comparisons.EQUAL, a.toTree(), b.toTree()
    ),
    Comparison_not_equal: (a, _, b) => CupidCompare(
        Comparisons.NOT_EQUAL, a.toTree(), b.toTree(),
    ),
    Comparison_less_than: (a, _, b) => CupidCompare(
        Comparisons.LESS_THAN, a.toTree(), b.toTree(),
    ),
    Comparison_less_or_equal: (a, _, b) => CupidCompare(
        Comparisons.LESS_OR_EQUAL, a.toTree(), b.toTree(),
    ),
    Comparison_greater_than: (a, _, b) => CupidCompare(
        Comparisons.GREATER_THAN, a.toTree(), b.toTree(),
    ),
    Comparison_greater_or_equal: (a, _, b) => CupidCompare(
        Comparisons.GREATER_OR_EQUAL, a.toTree(), b.toTree(),
    ),
    Comparison_and: (a, _, b) => CupidCompare(
        Comparisons.AND, a.toTree(), b.toTree(),
    ),
    Comparison_or: (a, _, b) => CupidCompare(
        Comparisons.OR, a.toTree(), b.toTree(),
    ),
}

export { useCompare }