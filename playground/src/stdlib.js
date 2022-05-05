// Types
import typedef from './../../stdlib/typedef.cupid?raw';
import integer from './../../stdlib/integer.cupid?raw';
import decimal from './../../stdlib/decimal.cupid?raw';

// Traits/implementations
import math from './../../stdlib/math.cupid?raw';
import compare from './../../stdlib/compare.cupid?raw';

export function read_file() {
	const stdlib = [typedef, integer, decimal, math, compare].join('\n\n');
	return stdlib;
}
