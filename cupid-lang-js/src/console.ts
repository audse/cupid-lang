export enum ConsoleColorModifier {
    Reset = '\x1b[0m',
    Bright = '\x1b[1m',
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

export enum ConsoleFgColorBright {
    Black = '\x1b[1;30m',
    Red = '\x1b[1;31m',
    Green = '\x1b[1;32m',
    Yellow = '\x1b[1;33m',
    Blue = '\x1b[1;34m',
    Magenta = '\x1b[1;35m',
    Cyan = '\x1b[1;36m',
    White = '\x1b[1;37m',
    Gray = '\x1b[1;90m',
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

export enum ConsoleBgColorBright {
    Black = '\x1b[1;40m',
    Red = '\x1b[1;41m',
    Green = '\x1b[1;42m',
    Yellow = '\x1b[1;43m',
    Blue = '\x1b[1;44m',
    Magenta = '\x1b[1;45m',
    Cyan = '\x1b[1;46m',
    White = '\x1b[1;47m',
    Gray = '\x1b[1;100m',
}

export type ConsoleColor = (
    ConsoleColorModifier
    | ConsoleFgColor
    | ConsoleFgColorBright
    | ConsoleBgColor
    | ConsoleBgColorBright
)

export function color () {
    return {
        underline: {
            red: () => `${ ConsoleColorModifier.Underline }${ colorString(ConsoleFgColor.Red) }`,
            yellow: () => `${ ConsoleColorModifier.Underline }${ colorString(ConsoleFgColor.Red) }`
        },
        dim: () => colorString(ConsoleColorModifier.Dim),
        red: () => colorString(ConsoleFgColor.Red),
        yellow: () => colorString(ConsoleFgColor.Yellow),
        bg: {
            red: () => colorString(ConsoleBgColor.Red),
            yellow: () => colorString(ConsoleBgColor.Yellow),
            bright: {}
        },
        bright: {
            red: () => colorString(ConsoleFgColorBright.Red),
        }
    }
}

function underlineString (): string {
    return `${ ConsoleColorModifier.Underline }%s${ ConsoleColorModifier.Reset }`
}

function colorString (color: ConsoleColor): string {
    return `${ color }%s${ ConsoleColorModifier.Reset }`
}

export function logColor (color: ConsoleColor, ...items: any[]) {
    console.log(`${ color }%s${ ConsoleColorModifier.Reset }`, ...items)
}

export function logFgRed (...items: any[]) {
    logColor(ConsoleFgColor.Red, ...items)
}