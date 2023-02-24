export namespace token {

    export type Line = number
    export type Column = number
    export type File = number

    export interface Position {
        line: Line
        column: Column
        index: number
    }

    export interface Span {
        start: Position
        end: Position
        file: File
    }

    export enum Type {
        Comment = 'comment',
        Symbol = 'symbol',
        Ident = 'ident',
        Number = 'number',
        Str = 'string',
        Reserved = 'reserved'
    }

    export interface Token extends Span {
        type: Type
        content: string
    }

    export enum Char {
        Space,
        Newline,
        Ident,
        Number,
        Symbol,
        Quote,
    }
}