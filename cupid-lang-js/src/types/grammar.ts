import { Option } from '@/types'

export namespace grmmr {

    export enum Base {
        Any = '@any',
        String = '@string',
        Int = '@int',
        Decimal = '@decimal',
        Ident = '@ident',
    }

    export enum Modifier {
        Multiple = '*',
        MoreThanOne = '+',
        Not = '!',
        Optional = '?',
        Ignore = '~'
    }

    export enum RuleModifier {
        PassThrough = '~',
        MatchStrings = 'match-strings'
    }

    export interface Rule {
        name: string
        alts: Group[]
        modifier: Option<RuleModifier>
        params: string[]
    }

    export interface Group {
        items: Item[]
        modifier: Option<Modifier>
    }

    export interface Item {
        content: string
        modifier: Option<Modifier>
        args: Item[]
    }
}