export function mapEntries<T extends object, R> (object: T, predicate: (key: keyof T, value: T[keyof T], i: number) => R): R[] {
    const results: R[] = []
    let i = 0
    for (const [key, value] of Object.entries(object)) {
        results.push(predicate(key as keyof T, value, i))
        i += 1
    }
    return results
}

export function filterObject<T extends object> (object: T, predicate: (key: keyof T, value: T[keyof T], i: number) => boolean): Partial<T> {
    const copy: Partial<T> = {}
    let i = 0
    for (const [key, value] of Object.entries(object)) {
        if (predicate(key as keyof T, value, i)) copy[key as keyof T] = value
        i += 1
    }
    return copy
}

export function filterObjectRecursive<T extends object> (object: T, predicate: (key: keyof T, value: T[keyof T], i: number) => boolean): Partial<Record<keyof T, Partial<T[keyof T]>>> {
    const copy: Partial<Record<keyof T, any>> = {}
    let i = 0
    for (const [key, value] of Object.entries(object)) {
        if (predicate(key as keyof T, value, i)) {
            copy[key as keyof T] = value
            if (typeof value === 'object' && value) copy[key as keyof T] = filterObjectRecursive(value, predicate)
        }
        i += 1
    }
    return copy
}

export function safeStringify (obj: any, indent = 2): string {
    let cache: any[] | null = []
    const retVal = JSON.stringify(
        obj,
        (key, value) =>
            cache && typeof value === 'object'
                ? value === null ? 'null'
                    : cache.includes(value) ? '[duplicate reference]' // undefined // Duplicate reference found, discard key
                        : cache.push(value) && value // Store value in our collection
                : value,
        indent
    )
    cache = null
    return retVal
}

export function debounce (func: any, wait: number) {
    let timeout: any
    return function () {
        // @ts-ignore
        const context = this
        const args = arguments
        const later = function () {
            timeout = null
            func.apply(context, args)
        }
        clearTimeout(timeout)
        timeout = setTimeout(later, wait)
    }
}

export function findMap<T, R> (array: T[], predicate: (item: T, index: number) => R | undefined): R | undefined {
    let i = 0
    for (const item of array) {
        const mappedItem = predicate(item, i)
        if (mappedItem !== undefined) return mappedItem
        i += 1
    }
}

export function filterMap<T, R> (array: T[], predicate: (item: T, index: number) => R | undefined): R[] {
    const items = []
    let i = 0
    for (const item of array) {
        const mappedItem = predicate(item, i)
        if (mappedItem !== undefined) items.push(mappedItem)
        i += 1
    }
    return items
}