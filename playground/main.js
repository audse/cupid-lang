import './style.css'

import init, { run_and_collect_logs } from './../cupid/pkg/cupid'

import { EditorState, basicSetup } from '@codemirror/basic-setup'
import { EditorView, keymap } from '@codemirror/view'
import { indentWithTab, defaultKeymap } from '@codemirror/commands'
import {
	indentUnit,
	foldNodeProp,
	foldInside,
	LRLanguage,
	LanguageSupport,
	syntaxHighlighting,
	HighlightStyle,
} from '@codemirror/language'
import { completeFromList } from '@codemirror/autocomplete'
import { buildParser } from '@lezer/generator'
import { styleTags, tags as t } from '@lezer/highlight'
import { tags } from '@codemirror/highlight'
import { jsonTree } from './src/jsonTree'
import grammar from './src/cupidgrammar.js'

const startCode = `
type person = [
    string name,
    int age
]

person jane = [
    name: 'Jane Doe',
    age: 34
]

# Try uncommenting this:
# log (jane.name)

`

window.addEventListener('load', async () => {
	await init()
	// const runButton = document.getElementById('cupid-run-button')
	const resultArea = document.getElementById('result-text')

	let currentResult = {}

	const doUpdate = update => {
		const currentText = update.state.doc.toJSON().join('\n')
		currentResult = run_and_collect_logs(currentText)
		resultArea.innerText = currentResult
		console.log(currentResult)
		makeTree()
	}

	let parser = buildParser(grammar)

	let parserWithMetadata = parser.configure({
		props: [
			styleTags({
				if: t.controlKeyword,
				else: t.controlKeyword,
				while: t.controlKeyword,
				for: t.controlKeyword,
				in: t.operatorKeyword,
				type: t.definitionKeyword,
				mut: t.modifier,
				Self: t.self,
				use: t.definitionKeyword,
				FunctionCall: t.function,
				Identifier: t.variableName,
				Boolean: t.bool,
				None: t.null,
				TypeName: t.typeName,
				AnyTypeName: t.className,
				PropertyName: t.propertyName,
				StructPropertyName: t.propertyName,
				String: t.string,
				LineComment: t.lineComment,
				MultiLineComment: t.blockComment,
				Integer: t.integer,
				Decimal: t.float,
				ArithmeticOperator: t.arithmeticOperator,
				CompareOperator: t.operatorKeyword,
				LogicalOperator: t.operator,
				Arrow: t.operatorKeyword,

				'( )': t.paren,
				'{ }': t.brace,
				'[ ]': t.squareBracket,
			}),
			foldNodeProp.add({
				Application: foldInside,
			}),
		],
	})

	let theme = HighlightStyle.define([
		{ tag: tags.variableName, class: 'variable-name' },
		{ tag: tags.string, class: 'string' },
		{ tag: tags.definitionKeyword, class: 'definition-keyword' },
		{ tag: tags.controlKeyword, class: 'control-keyword' },
		{ tag: tags.number, class: 'number' },
		{ tag: tags.propertyName, class: 'property-name' },
		{ tag: tags.typeName, class: 'builtin-type-name' },
		{ tag: tags.comment, class: 'comment' },
		{ tag: tags.className, class: 'class-name' },
		{ tag: tags.self, class: 'self-keyword' },
		{
			tag: [
				tags.operator,
				tags.arithmeticOperator,
				tags.operatorKeyword,
				tags.compareOperator,
				tags.operatorKeyword,
			],
			class: 'operator',
		},
	])

	const cupidLang = LRLanguage.define({
		parser: parserWithMetadata,
		languageData: {
			commentTokens: { line: '#' },
		},
	})

	const cupidCompletion = cupidLang.data.of({
		autocomplete: completeFromList([
			{ label: 'type', type: 'keyword' },
			{ label: 'use', type: 'keyword' },
			{ label: 'string', type: 'type' },
			{ label: 'char', type: 'type' },
			{ label: 'bool', type: 'type' },
			{ label: 'int', type: 'type' },
			{ label: 'dec', type: 'type' },
			{ label: 'nothing', type: 'type' },
			{ label: 'array', type: 'type' },
			{ label: 'map', type: 'type' },
			{ label: 'none', type: 'literal' },
			{ label: 'self', type: 'literal' },
			{ label: 'true', type: 'literal' },
			{ label: 'false', type: 'literal' },
		]),
	})

	const lang = new LanguageSupport(cupidLang, [cupidCompletion])

	// runButton.addEventListener('click', event => {
	//     event.preventDefault()
	//     event.stopPropagation()
	// })

	let view = new EditorView({
		state: EditorState.create({
			doc: startCode,
			extensions: [
				basicSetup,
				keymap.of([indentWithTab, defaultKeymap]),
				EditorState.tabSize.of(4),
				indentUnit.of('	'),
				EditorView.updateListener.of(doUpdate),
				EditorView.theme({}, { dark: true }),
				lang,
				syntaxHighlighting(theme),
			],
		}),
		parent: document.body.querySelector('#editor'),
	})

	let result = document.getElementById('result-text')

	const makeTree = () => {
		let tree = jsonTree.create(currentResult.semantics.File, result)
		tree.expand(node => {
			if (node.label !== 'token') {
				return true
			} else {
				return false
			}
		})
	}
})
