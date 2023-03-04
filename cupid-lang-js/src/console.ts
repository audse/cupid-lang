export enum ConsoleColorModifier {
    Italic = '\x1b[3m',
    Reset = '\x1b[0m',
    Bold = '\x1b[1m',
    BoldUnderline = '\x1b[1;4m',
    Dim = '\x1b[2m',
    Underline = '\x1b[4m',
    Blink = '\x1b[5m',
    Reverse = '\x1b[7m',
    Hidden = '\x1b[8m',
}

export enum ConsoleFgColor {
    Black = '\x1b[30m',
    Red = '\x1b[31m',
    Green = '\x1b[32m',
    Yellow = '\x1b[33m',
    Blue = '\x1b[34m',
    Magenta = '\x1b[35m',
    Cyan = '\x1b[36m',
    White = '\x1b[37m',
    Gray = '\x1b[90m',
}

export enum ConsoleBgColor {
    Black = '\x1b[40m',
    Red = '\x1b[41m',
    Green = '\x1b[42m',
    Yellow = '\x1b[43m',
    Blue = '\x1b[44m',
    Magenta = '\x1b[45m',
    Cyan = '\x1b[46m',
    White = '\x1b[47m',
    Gray = '\x1b[100m',
}

export type ConsoleColor = (
    ConsoleColorModifier
    | ConsoleFgColor
    | ConsoleBgColor
)

function makeFunctions (palette: typeof ConsoleFgColor | typeof ConsoleBgColor, ...colors: ConsoleColor[]) {
    return {
        black: (str: string) => colorString(str, ...colors, palette.Black),
        red: (str: string) => colorString(str, ...colors, palette.Red),
        yellow: (str: string) => colorString(str, ...colors, palette.Yellow),
        green: (str: string) => colorString(str, ...colors, palette.Green),
        blue: (str: string) => colorString(str, ...colors, palette.Blue),
        magenta: (str: string) => colorString(str, ...colors, palette.Magenta),
        cyan: (str: string) => colorString(str, ...colors, palette.Cyan),
        white: (str: string) => colorString(str, ...colors, palette.White),
        gray: (str: string) => colorString(str, ...colors, palette.Gray),
    } as const
}

export const color = {
    underline: makeFunctions(ConsoleFgColor, ConsoleColorModifier.Underline),
    dimmed: makeFunctions(ConsoleFgColor, ConsoleColorModifier.Dim),
    bold: makeFunctions(ConsoleFgColor, ConsoleColorModifier.Bold),
    boldUnderline: makeFunctions(ConsoleFgColor, ConsoleColorModifier.BoldUnderline),
    italic: makeFunctions(ConsoleFgColor, ConsoleColorModifier.Italic),
    bg: makeFunctions(ConsoleBgColor),
    dim: (str: string) => colorString(str, ConsoleColorModifier.Dim),
    ...makeFunctions(ConsoleFgColor)
}

export function colorString (string: string, ...colors: ConsoleColor[]): string {
    return `${ colors.join('') }${ string }${ ConsoleColorModifier.Reset }`
}