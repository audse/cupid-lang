const loopTests = test => {
    // While
    test('x = 0', 0)
    test('while x < 5 => x = x + 1', 5)
    test('y = 10', 10)
    test('while y < 10 => y = y + 1', null)

    // For in
    test('for x in [1, 2, 3] => x', 3)
    test('for i, y in [1, 2, 3] => [i, y]', [2, 3])
    test('for i, key, val in [a: "x", b: "y", c: "z"] => [i, key, val]', [2, 'c', 'z'])
    test('for val in [a: 0, b: 1] => val', 1)
}

export default loopTests