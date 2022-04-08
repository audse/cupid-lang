
const assignmentTests = test => {
    test('x = 10', 10)
    test('x', 10)
    test('x * 2', 20)
    test('x = *** false *** true', true)
}

export default assignmentTests