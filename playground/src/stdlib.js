import typedef from './../../stdlib/typedef.cupid?raw';
import integer from './../../stdlib/integer.cupid?raw';
import decimal from './../../stdlib/decimal.cupid?raw';

export function read_file() {
	const stdlib = [typedef, integer, decimal].join('\n\n');
	return stdlib;
}
