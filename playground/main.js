import './static/normalize.css';
import './static/main.css';
import './static/dropdown.css';

import init, { run_and_collect_logs } from './../cupid/pkg/cupid';

import { EditorState, basicSetup } from '@codemirror/basic-setup';
import { EditorView, keymap } from '@codemirror/view';
import { indentWithTab, defaultKeymap } from '@codemirror/commands';
import {
	indentUnit,
	foldNodeProp,
	foldInside,
	LRLanguage,
	LanguageSupport,
	syntaxHighlighting,
	HighlightStyle,
} from '@codemirror/language';
import { completeFromList } from '@codemirror/autocomplete';
import { buildParser } from '@lezer/generator';
import { styleTags, tags as t } from '@lezer/highlight';
import { tags } from '@codemirror/highlight';
import { jsonTree } from './src/jsonTree';
import grammar from './src/cupidgrammar.js';
import { semantics, parse, scope, serializeJson } from './src/outputTree.js';
import { code, bindExampleButtons } from './src/examples.js';

const Tabs = {
	OUTPUT: 0,
	TREE: 1,
	PARSE: 2,
	SCOPE: 3,
};

window.addEventListener('load', async () => {
	await init();
	const outputElement = document.getElementById('result-text');
	const outputButton = document.getElementById('output-button');
	const treeButton = document.getElementById('tree-button');
	const parseButton = document.getElementById('parse-button');
	const scopeButton = document.getElementById('scope-button');

	let currentText = '';
	let currentResult = {};

	let tab = Tabs.OUTPUT;

	const showOutput = () => {
		tab = Tabs.OUTPUT;
		outputButton.classList.add('active');
		treeButton.classList.remove('active');
		parseButton.classList.remove('active');
		outputElement.innerHTML = `
            ${createOutput(currentResult.values)}
            ${currentResult.errors.map(createError).join('\n')}
        `;
	};

	const createOutput = values =>
		values.reduce((prev, value = []) => {
			const isObject = typeof value[1] === 'object';
			const isLog = isObject && 'Log' in value[1];
			const html = isLog
				? value[0]
				: `<span style="opacity: 0.5">${value[0]}</span>`;
			return prev + html + '<br />';
		}, '');

	const createError = error => {
		const lines = currentText.split('\n');
		const line = lines[error.line - 1];
		const lineAbove =
			lines.length >= error.line - 2 ? lines[error.line - 2] : '';
		const length = error.source.length;
		const space = Array.from({ length: error.index }, () => '&nbsp;').join(
			''
		);
		const underline = Array.from({ length: length }, () => '^');
		const lineNumber = `&nbsp;&nbsp;${error.line}`;
		const lineNumberBelow = `&nbsp;&nbsp;${error.line + 1}`;
		const lineNumberAbove = `&nbsp;&nbsp;${error.line - 1}`;

		return `
            <div class="result-error">
                <b>
                    <span class="red">error:</span> 
                    ${error.message}
                </b>
                <br />
                <i class="muted">
                    &nbsp;&nbsp;-->&nbsp; 
                    line ${error.line}:${error.index}
                    at \`<b class="yellow">${error.source}</b>\`
                </i>
                <div style="padding: 14px 0 0 14px">
                    <span class="muted">${lineNumberAbove} | ${lineAbove}</span>
                    <br />
                    <span class="muted">${lineNumber} |</span> ${line}</span>
                    <br />
                    <span class="muted">${lineNumberBelow} | </span>
                    <span class="red">
                        ${space}${underline.join('')}
                    </span>
                </div>
                <br />
                <b>additional context</b>: 
                <span class="muted">
                    ${error.context}
                </span>
            </div>
        `;
	};

	const showTree = () => {
		tab = Tabs.TREE;
		outputButton.classList.remove('active');
		treeButton.classList.add('active');
		parseButton.classList.remove('active');
		scopeButton.classList.remove('active');
		outputElement.innerText = '';
		semantics.makeTree(currentResult, outputElement);
	};

	const showParse = () => {
		tab = Tabs.PARSE;
		outputButton.classList.remove('active');
		treeButton.classList.remove('active');
		parseButton.classList.add('active');
		parseButton.classList.remove('active');
		outputElement.innerText = '';
		parse.makeTree(currentResult, outputElement);
	};

	const showScope = () => {
		tab = Tabs.SCOPE;
		outputButton.classList.remove('active');
		treeButton.classList.remove('active');
		parseButton.classList.remove('active');
		scopeButton.classList.add('active');
		outputElement.innerText = '';
		scope.makeTree(currentResult, outputElement);
	};

	outputButton.addEventListener('click', showOutput);
	treeButton.addEventListener('click', showTree);
	parseButton.addEventListener('click', showParse);
	scopeButton.addEventListener('click', showScope);

	const debounce = (callback, wait) => {
		let timeoutId = null;
		return (...args) => {
			window.clearTimeout(timeoutId);
			timeoutId = window.setTimeout(() => {
				callback.apply(null, args);
			}, wait);
		};
	};

	const doUpdate = update => {
		currentText = update.state.doc.toJSON().join('\n');
		currentResult = run_and_collect_logs(currentText);
		console.log(currentResult);

		switch (tab) {
			case Tabs.OUTPUT:
				showOutput();
				break;
			case Tabs.TREE:
				showTree();
				break;
			case Tabs.PARSE:
				showParse();
				break;
			case Tabs.SCOPE:
				showScope();
				break;
			default:
				break;
		}
	};

	let parser = buildParser(grammar);

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
				with: t.definitionKeyword,
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
	});

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
		{ tag: tags.bool, class: 'boolean' },
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
	]);

	const cupidLang = LRLanguage.define({
		parser: parserWithMetadata,
		languageData: {
			commentTokens: { line: '#' },
		},
	});

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
	});

	const lang = new LanguageSupport(cupidLang, [cupidCompletion]);

	const debounceUpdate = debounce(doUpdate, 250);

	let view = new EditorView({
		state: EditorState.create({
			doc: code[0],
			extensions: [
				basicSetup,
				keymap.of([indentWithTab, defaultKeymap]),
				EditorState.tabSize.of(4),
				indentUnit.of('	'),
				EditorView.updateListener.of(debounceUpdate),
				EditorView.theme({}, { dark: true }),
				lang,
				syntaxHighlighting(theme),
			],
		}),
		parent: document.body.querySelector('#editor'),
	});

	bindExampleButtons((code, event) => {
		event.preventDefault();
		let end = view.state.doc.length;
		view.dispatch({
			changes: { from: 0, to: end, insert: code },
		});
		const dropdown = document.getElementById('dropdown-label');
		dropdown.innerText = event.currentTarget.innerText;
	});
});
