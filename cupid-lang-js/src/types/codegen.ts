
export type ObjectLiteral = { [key: string | number]: Literal }
export type Literal = string | number | boolean | null | Literal[] | ObjectLiteral
export type GeneratedString = string

export interface Config {
    export?: boolean
}

export interface NamespaceParams extends NamespaceConfig {
    name: string,
    statements: GeneratedString | GeneratedString[],
}

export interface NamespaceConfig extends Config { }

export interface TypeConfig extends Config {
    and?: string | string[]
}

export interface InterfaceConfig { }

export interface VariableConfig extends Config {
    type?: string,
}

export interface FunctionConfig extends VariableConfig {
    async?: boolean
}

export interface TypeParams<V> extends TypeConfig {
    name: string,
    value: V,
}

export interface AnonFuncParams extends FunctionConfig {
    params: GeneratedString[],
    statements: GeneratedString | GeneratedString[],
}

export interface FuncParams extends AnonFuncParams {
    name: string,
    statements: GeneratedString[],
}

export interface IfStmtParams {
    compare: GeneratedString
    thenDo: GeneratedString | GeneratedString[]
    elseDo?: GeneratedString | GeneratedString[]
    elseIf?: IfStmtParams[]
}

export interface VariableParams extends VariableConfig {
    name: string
    value: GeneratedString
}

export interface AssignParams {
    name: string
    type?: GeneratedString
    value?: GeneratedString
}

export type Formatter<F = string> = (str: F) => F | string
