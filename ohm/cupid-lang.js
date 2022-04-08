import ohm from 'ohm-js'
import fs from 'fs'

import {
    CupidSemantics,
    CupidTester
} from './index.js';

const contents = fs.readFileSync('./syntax/cupid-lang.ohm', 'utf-8')
const CupidLang = ohm.grammar(contents)

const semantics = CupidLang.createSemantics()
const makeTree = CupidSemantics(semantics).makeTree

CupidTester(CupidLang, makeTree)