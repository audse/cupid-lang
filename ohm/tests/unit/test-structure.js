const structureTests = test => {

    // Lists
    test('[1, 2, 3]', [1, 2, 3])
    test('["a", "b", "c"]', ['a', 'b', 'c'])
    test('{ x = 1 y = 2 z = 3 }', 3)
    test('[x, y, z]', [1, 2, 3])
    test('[x, 2, 3]', [1, 2, 3])
    test('[1, 2, 3].0', 1)
    test('[1, 2, 3].4', null)

    // Dictionaries
    test('[a: 1, b: 2, c: 3]', { a: 1, b: 2, c: 3 })
    test('["a": [1, 2, 3]]', { a: [1, 2, 3] })
    test('[a: 1, b: 2, c: 3].a', 1)
    test('let my_dict = [a: 1, b: 2]', { a: 1, b: 2 })
    test('my_dict.a', 1)
    test('let dict_copy = [c: my_dict.a + my_dict.b]', { c: 3 })
    test('my_dict.c', null)
    test('my_dict.a = 2', 2)
    test('my_dict.a', 2)

    // Tuples
    test('let my_tuple = ("a", "b", "c")', ['a', 'b', 'c'])
    test('(true, false)', [true, false])
    test('let accessor = 1', 1)
    test('my_tuple[accessor]', 'b')
    test('(1, 2, 3).accessor', 2)
    test('(1, 2, 3)[accessor]', 2)

    // Range
    test('[0..3]', [0, 1, 2, 3])
    test('0[..3]', [1, 2, 3])
    test('0[..]3', [1, 2])
    test('[0..]3', [0, 1, 2])
    test('[accessor..3]', [1, 2, 3])
    test('[0..10].accessor', 1)
}

export default structureTests