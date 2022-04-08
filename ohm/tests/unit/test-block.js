
const blockTests = test => {
    test(`{ x = 1 y = 10 x + y }`, 11)
    test(`if 10 is 10 { 10 + 10 } else { 10 - 10 }`, 20)
    test(`if 10 not 10 { 10 + 10 } else { 10 - 10 }`, 0)
    test(`if 1 < 10 => true else => false`, true)
    test(`if 1 => true`, true)
}

export default blockTests