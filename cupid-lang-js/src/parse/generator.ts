import { GeneratedString, grmmr, Option } from '@/types'
import * as gen from '@/codegen'

function isPassThroughRule (rule: grmmr.Rule) {
    return rule.modifier === grmmr.RuleModifier.PassThrough
}

function isPassThroughItem (item: grmmr.Item) {
    return item.modifier === grmmr.Modifier.Ignore
}

function isMultipleItem (item: grmmr.Item) {
    return item.modifier === grmmr.Modifier.Multiple
}

function isMoreThanOneItem (item: grmmr.Item) {
    return item.modifier === grmmr.Modifier.MoreThanOne
}

export namespace generator {

    export function generate (name: string, grammar: grmmr.Rule[]): GeneratedString {
        return gen.reindent([
            `\nimport { Option, token, Node, NodeParser } from '@/types'`,
            `import { getNodeArray, node, modifier } from '@/parse/utils'`,
            `import { TokenParser } from '@/parse/parse'\n`,
            gen.headerComment('AUTOMATICALLY GENERATED - DO NOT EDIT'),
            gen.namespace({
                name,
                statements: grammar.map(rule),
                export: true
            })
        ].join('\n'))
    }

    export function rule (rule: grmmr.Rule): GeneratedString {
        if (rule.modifier === grmmr.RuleModifier.MatchStrings) return matchRule(rule)
        const params = [
            gen.assign({ name: 'parser', type: 'TokenParser' }),
            ...rule.params.map(name => gen.assign({ name, type: 'NodeParser' }))
        ]
        const ruleName = rule.name.toLowerCase()
        const groupName = `${ ruleName }Group`
        const groups = rule.alts.map(group).join('\n?? ')
        const returns = isPassThroughRule(rule)
            ? `return getNodeArray(${ groupName })`
            : `return { 
                name: '${ rule.name }',
                items: getNodeArray(${ groupName }),
            }`
        return gen.func({
            name: ruleName,
            params,
            statements: [
                gen.constant({ name: groupName, value: groups }),
                gen.ifStmt({ compare: groupName, thenDo: returns }),
                'return null'
            ],
            type: isPassThroughRule(rule) ? `Option<Node[]>` : `Option<Node>`,
            export: true,
        })
    }

    function matchRule (rule: grmmr.Rule): GeneratedString {
        const strings: string[] = rule.alts.map(alt => alt.items.map(item => item.content).flat()).flat()
        const ruleName = rule.name.toLowerCase()
        const setName = `${ ruleName }Accepted`
        return [
            gen.constant({
                name: setName,
                type: 'Set<string>',
                value: `new Set([${ strings.join(', ') }])`
            }),
            gen.func({
                name: ruleName,
                params: [gen.assign({ name: 'parser', type: 'TokenParser' })],
                statements: [`return node.string(parser.matchOneSet(${ setName }))`],
                type: 'Option<Node>',
                export: true,
            })
        ].join('\n')
    }

    function group (group: grmmr.Group, i: number): GeneratedString {
        switch (group.modifier) {
            case grmmr.Modifier.Multiple: return `modifier.multiple(parser, parser => ${ groupBody(group) }).flat()`
            case grmmr.Modifier.MoreThanOne: return `modifier.moreThanOne(parser, parser => ${ groupBody(group) }).flat()`
            case grmmr.Modifier.Optional: return `modifier.optional(parser, parser => ${ groupBody(group) })`
            case grmmr.Modifier.Not: return `modifier.negative(parser, parser => ${ groupBody(group) })`
            default: return groupBody(group)
        }
    }

    function groupBody (group: grmmr.Group): GeneratedString {
        if (group.items.length === 1) return `(${ item(group.items[0]) })(parser)`
        return `parser.chain(\n${ group.items.map(item).join(',\n') }\n)`
    }

    function item (item: grmmr.Item): GeneratedString {
        const passThrough = isPassThroughItem(item)
        const stmt = (
            funcItemValue(item, passThrough)
            || stringItemValue(item, passThrough)
            || builtinItemValue(item, passThrough)
            || identItemValue(item, passThrough)
            || `() => throw 'item didn\'t match any available functions:\n${ JSON.stringify(item, null, 2) }'`
        )
        const body = gen.anonFunc({
            params: ['parser'],
            statements: passThrough ? `${ stmt } ? true : null`
                : stmt,
            type: passThrough
                ? `Option<boolean>`
                : `Option<Node | Node[]>`
        })
        switch (item.modifier) {
            case grmmr.Modifier.Multiple: return `parser => modifier.multiple(parser, ${ body }).flat()`
            case grmmr.Modifier.MoreThanOne: return `parser => modifier.moreThanOne(parser, ${ body }).flat()`
            case grmmr.Modifier.Optional: return `parser => modifier.optional(parser, ${ body })`
            case grmmr.Modifier.Not: return `parser => modifier.negative(parser, ${ body })`
            default: return body
        }
    }

    function funcItemValue (itm: grmmr.Item, isPassThrough: boolean = false): Option<GeneratedString> {
        if (!itm.args.length) return null
        const args = ['parser', ...itm.args.map(item)]
        return `${ itm.content.toLowerCase() }(\n${ args.join(',\n') }\n)`
    }

    function stringItemValue (item: grmmr.Item, isPassThrough: boolean = false): Option<GeneratedString> {
        const regex = /['"`][^'"`]*['"`]/
        if (regex.test(item.content)) return (
            isPassThrough ? `parser.match(${ item.content })`
                : `node.string(parser.match(${ item.content }))`
        )
        return null
    }

    function builtinItemValue (item: grmmr.Item, isPassThrough: boolean = false): Option<GeneratedString> {
        const regex = /\@(?<content>.*)/
        const match = item.content.match(regex)
        if (match?.groups?.content) {
            const content = match.groups.content
            return (
                isPassThrough ? `parser.${ content }()`
                    : `node.${ content }(parser.${ content }())`
            )
        }
        return null
    }

    function identItemValue (item: grmmr.Item, isPassThrough: boolean = false): Option<GeneratedString> {
        return `${ item.content.toLowerCase() }(parser)`
    }
}