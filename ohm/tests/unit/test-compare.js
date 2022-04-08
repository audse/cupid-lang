
const compareTests = test => {
    test('10 is 20', false)
    test('10 == 10', true)
    test('10 not 20', true)
    test('10 != 10', false)
    test('10 and 20', true)
    test('0 && 0', false)
    test('true or false', true)
    test('10 < 20', true)
    test('10 <= 5', false)
    test('20 > 10', true)
    test('20 >= 100', false)
    test('10 * 10 + 5 > 2', true)
}

export default compareTests