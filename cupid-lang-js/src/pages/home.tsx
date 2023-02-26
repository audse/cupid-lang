import { useEffect, useRef, useState } from 'react'
import { filterObjectRecursive, safeStringify } from '@/utils'
import { create, Tree } from '@/components/json-tree'
import { test } from '@/passes/test/tests'
import { reindent } from '@/codegen'
import { Err, format, FormatJsx } from '@/error'
import { Node, Result } from '@/types'

const presetClosure = reindent(`
    type int = type! int
    type str = type! str

    let counter = n : int => {
        let mut i = n
        let counter = () => {
            i = i + 1
            i
        }
        counter
    }

    let count = counter (0)
    count()
    count()
`.trim())

const presetFib = reindent(`
    type int = type! int
    let fib = n : int -> int => {
        if n <= 0 { n }
        else { fib (n - 1) + fib (n - 2) }
    }
    fib (10)
`.trim())


const presetMath = reindent(`
    type int = type! int
    type decimal = type! decimal
    let x : int = 10
    let y : int = 20
    let z : int = 2
    x + (y * z)
`.trim())

const presetTypedef = reindent(`
    type type = type! type
    type int = type! int

    type point = t => type! [
        x : t
        y : t
    ]

    type my-point-type = point [int]

    let my-point = [
        x : 1,
        y : 1
    ]

    my-point\\x

    let operations = [
        add : (a : int, b : int) => a + b
    ]

    (operations\\add)(1, 2)
`.trim())

const presetFunction = reindent(`
    type type = type! type
    type int = type! int
    type decimal = type! decimal

    type number = sum! [
        i : int
        d : decimal
    ]

    let add = a : int, b : int => a + b

    let a = add (5, 5)
    -- let b = add (1.5, 2.25)
    -- add (a, b)
`.trim())

function JsonTree ({ content }: { content: any }) {
    const container = useRef<HTMLElement>(null)
    const [tree, setTree] = useState<Tree | null>(null)

    const getContent = () => JSON.parse(
        safeStringify(
            filterObjectRecursive(content, key => !['scope', 'source'].includes(key.toString()))
        )
    )

    function expand () {
        if (tree) tree.expand((node: any) => !(
            [
                'token',
                'tokens',
                'start',
                'end',
                'scope'
            ].includes(node.label.toString().toLowerCase())
        ))
    }

    useEffect(() => {
        if (container.current && tree === null) {
            container.current.innerHTML = ''
            setTree(create(getContent(), container.current))
            expand()
        }
    }, [container.current])

    useEffect(() => {
        if (tree) {
            tree.loadData(getContent())
            expand()
        }
    }, [content])

    expand()

    return (<section ref={ container }></section>)
}

export function Home () {

    const [content, setContent] = useState(presetTypedef)
    const [showResults, setShowResults] = useState(true)
    const [showAst, setShowAst] = useState(false)

    let tree: any[] = []
    let results: any[] = []
    let nodes: Node[] = []
    let error: Err | null = null


    try {
        const compiled = test(content)
        results = compiled.results
        tree = compiled.tree
        nodes = compiled.env
    } catch (err: any) {
        console.log(err)
        error = (typeof err.error === 'object' && err.error && 'err' in err.error) ? err.error.err as Err : null
        results = []
        tree = err.tree
        nodes = err.env
    }

    return (
        <main>
            <h1>Home</h1>

            <div style={ { display: 'flex', gap: '2rem' } }>
                <section style={ { flex: 1, maxWidth: '50%' } }>
                    {/* <button onClick={ () => setContent(presetFib) }>ex. Fibonacci</button> */ }
                    <button onClick={ () => setContent(presetClosure) }>ex. Closure</button>
                    <button onClick={ () => setContent(presetMath) }>ex. Math</button>
                    <button onClick={ () => setContent(presetTypedef) }>ex. Typedef</button>
                    <button onClick={ () => setContent(presetFunction) }>ex. Function</button>
                    <textarea
                        rows={ 30 }
                        value={ content }
                        onChange={ event => setContent(event.currentTarget.value) } />
                </section>
                <section style={ { flex: 1, maxWidth: '50%' } }>
                    <label style={ { marginRight: '1rem' } }>
                        <input type='checkbox' onClick={ () => setShowResults(!showResults) } defaultChecked={ showResults } />
                        Show Results
                    </label>
                    <label>
                        <input type='checkbox' onClick={ () => setShowAst(!showAst) } defaultChecked={ showAst } />
                        Show AST
                    </label>
                    { error && <>
                        <h2>Errors</h2>
                        <pre><FormatJsx files={ [content] } error={ error } env={ nodes } /></pre>
                    </> }
                    { showResults && <>
                        <h2>Results</h2>
                        <JsonTree content={ results } />
                    </> }
                    { showAst && <>
                        <h2>AST</h2>
                        <JsonTree content={ tree } />
                    </> }
                </section>
            </div>

        </main>
    )
}