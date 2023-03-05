import { resolve } from 'path'
import { Cupid } from '@/pipeline'
import { Expr } from '@/ast'

const cupid = new Cupid()

const path = resolve(process.argv.at(-1) || './apps/main.cupid')

const stdlibPaths = [
    resolve('./apps/primitives.cupid'),
    resolve('./apps/int.cupid'),
    resolve('./apps/decimal.cupid'),
    resolve('./apps/type.cupid'),
]

const stdlib: [string, string][] = []
for (const libPath of stdlibPaths) {
    stdlib.push([libPath, await Bun.file(libPath).text()])
}

cupid.addFiles(
    ...stdlib,
    [path, await Bun.file(path).text()]
)

const results = cupid.interpret()
const values = results?.map(result => result instanceof Expr ? result.report() : result)
values?.map(val => console.log(val))
