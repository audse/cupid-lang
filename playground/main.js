import './style.css'

import init, { run_and_collect_logs } from './../cupid/pkg/cupid.js'
init()

import { EditorState, basicSetup } from '@codemirror/basic-setup'
import { EditorView, keymap } from '@codemirror/view'
import { indentWithTab } from '@codemirror/commands'
import { indentUnit } from '@codemirror/language'

const startCode = `type char
type bool
type array
type map
type fun
type nothing
type string
type int
type dec

type person = [
	string name,
	int age
]

person jane = [
	name: 'Jane Doe',
	age: 34
]
`

window.addEventListener('load', event => {
    const runButton = document.getElementById('cupid-run-button')
    const resultArea = document.getElementById('result')

    const doUpdate = update => {
        console.log('here')
        const currentText = update.state.doc.toJSON().join('\n')
        let val = run_and_collect_logs(currentText)
        resultArea.innerText = val
    }

    runButton.addEventListener('click', event => {
        event.preventDefault()
        event.stopPropagation()
        doUpdate(view)
    })

    let view = new EditorView({
        state: EditorState.create({
            doc: startCode,
            extensions: [
                basicSetup,
                keymap.of([indentWithTab]),
                EditorState.tabSize.of(4),
                indentUnit.of('	'),
                EditorView.updateListener.of(doUpdate),
            ],
        }),
        parent: document.body.querySelector('#editor'),
    })
})
