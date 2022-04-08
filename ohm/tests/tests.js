import { strict as assert } from 'assert'
import fs from 'fs'
import { CupidScope, CupidSymbol } from './../tree/index.js'

import { 
    mathTests, 
    assignmentTests, 
    blockTests, 
    compareTests, 
    loopTests, 
    functionTests,
    structureTests,
} from './unit/index.js'

const CupidTester = (CupidLang, makeTree) => {

    const SCOPE = CupidScope(null)

    const toSyntax = string => {
        return string.replace(/[\{]/g, '[').replace(/[\}]/g, ']')
    }
    const mapArgs = arg => {
        if (typeof arg === 'object' && arg) {
            if ('value' in arg) return `${ arg.value }`
            else if ('getValue' in arg) return toSyntax(JSON.stringify(arg.getValue()))
        } else if (!arg) return 'none'
        return `${ arg }`
    }

    SCOPE.setSymbol(
        CupidSymbol('log'),
        (...args) => {
            let newArgs = Array.isArray(args) ? args : [args]
            console.log(...newArgs.map(arg => mapArgs(arg)))
            return args
        }
    )

    SCOPE.setSymbol(
        CupidSymbol('logline'),
        (...args) => {
            let newArgs = Array.isArray(args) ? args : [args]
            process.stdout.write(...newArgs.map(arg => mapArgs(arg)))
            return args
        }
    )


    SCOPE.setSymbol(
        CupidSymbol('loglines'),
        (...args) => {
            let newArgs = Array.isArray(args) ? args : [args]
            let lines = newArgs.map(arg => mapArgs(arg)).join(' ')
            process.stdout.write(lines)
            return args
        }
    )

    const test = input => {
        const match = CupidLang.match(input, 'Expression')

        if (match.failed()) {
            return console.log('Failed to match.', input, match.message)
        }

        let tree, result
        try {
            tree = makeTree(match).toTree()
            result = tree.resolve(SCOPE)
        } catch (error) {
            console.log('Result:', result)
            console.log('Tree:', tree)
            console.log('Error:', error)
            console.log('\n\n\n')
        }
        return result
    }

    const testAssertQuick = (input, answer) => {
        const result = test(input)
        tryAssert(result, input, answer)
        if (result) process.stdout.write('.')
    }

    const testAssert = (input, answer) => {
        console.log(`"${ input }"`, 'is', answer)
        tryAssert(result, input, answer)
        console.log('Success!')
    }

    const tryAssert = (result, input, answer) => {
        try {
            assert.deepEqual(result.isEqual(answer, SCOPE), true)
        } catch (error) {
            console.log('\nFailed test!')
            console.log('Input:', input)
            console.log('Answer:', answer)
            console.log('Result:', result)
            console.log('Error:', error)
            if (result && 'getValue' in result) console.log('Result value:', result.getValue())
        }
    }

    const testFile = filePath => {
        const data = fs.readFileSync(filePath, 'utf8')
        console.log('\n')

        const match = CupidLang.match(data, 'File')

        if (match.failed()) {
            return console.log('Failed to match.', match.message)
        }

        const tree = makeTree(match).toTree()
        const result = tree.forEach(expression => expression.resolve(SCOPE))

        console.log('\n')
    }

    mathTests(testAssertQuick)
    compareTests(testAssertQuick)
    blockTests(testAssertQuick)
    loopTests(testAssertQuick)
    assignmentTests(testAssertQuick)
    functionTests(testAssertQuick)
    structureTests(testAssertQuick)

    testFile('./tests/integration/main.cupid')
}

export { CupidTester }