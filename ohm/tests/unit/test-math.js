const mathTests = test => {
    // integer
    test('123', 123)

    // float
    test('99.99', 99.99)

    // big integer
    test('1_234_567', 1234567)
    
    test('10+10', 20)
    test('20-10', 10)
    test('10*10', 100)
    test('100 / 10', 10)
    test('10 + 100 / 10 - 5', 15)
    test('(10 + 5) / 5', 3)
    test('2 ^ 3', 8)
    test('10 % 5', 0)
    
    // string math
    test('"abc" + "xyz"', 'abcxyz')
}

export default mathTests