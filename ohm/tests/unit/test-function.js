
const functionTests = test => {
    // test('logline(".")', ['.'])
    test('x = 10', 10)
    test('y = 10', 10)
    test('{ fun add (a, b) { a + b } add(x, y) }', 20)

    test('(1 => x => x + 1)', 2)
}

export default functionTests