import { jsonTree } from './jsonTree';

const semantics = {
	hideTokens: true,

	filterJson: expression => {
		if (typeof expression === 'object') {
			Object.entries(expression).forEach(([name, node]) => {
				delete expression[name];
				if (!semantics.shouldRemoveJson(name, node)) {
					const newExpression = semantics.collapseJson(
						name,
						semantics.filterJson(node)
					);
					expression[newExpression[0]] = newExpression[1];
				}
			});
			return expression;
		} else if (Array.isArray(expression)) {
			return expression.map(semantics.filterJson);
		} else {
			return expression;
		}
	},

	collapseJson: (name, expression) => {
		if (['node', 'block'].includes(name.toLowerCase())) {
			const entries = Object.entries(expression);
			return entries.length ? entries[0] : {};
		}

		if (name.toLowerCase() === 'symbol' && expression.identifier) {
			Object.entries(expression.identifier).forEach(([key, val]) => {
				expression[key] = val;
			});
			delete expression.identifier;
		}
		return [name, expression];
	},

	shouldRemoveJson: (name, _) => {
		if (
			semantics.hideTokens &&
			['token', 'tokens'].includes(name.toLowerCase())
		) {
			return true;
		} else {
			return false;
		}
	},

	collapseNodes: node => {
		return !['token', 'tokens', 'operator'].includes(node.label);
	},

	makeTree: (result, el) => {
		const semanticsJson = result.semantics?.map(semantics.filterJson);
		const tree = jsonTree.create(semanticsJson, el);
		tree.expand(semantics.collapseNodes);
	},
};

const parse = {
	hideTokens: true,

	filterJson: node => {
		if (Array.isArray(node)) {
			return node.map(parse.filterJson);
		} else if (typeof node === 'object') {
			const newNode = {};
			if (node.children && node.children.length === 0) {
				delete node.children;
			}
			let keys = Object.keys(node);
			keys.sort()
				.reverse()
				.forEach(name => {
					const n = node[name];
					if (!parse.shouldRemoveJson(name, n)) {
						newNode[name] = parse.filterJson(n);
					}
				});
			return newNode;
		} else if (Array.isArray(node)) {
			return node.map(parse.filterJson);
		} else {
			return node;
		}
	},

	shouldRemoveJson: (name, _) => {
		if (
			parse.hideTokens &&
			['token', 'tokens'].includes(name.toLowerCase())
		) {
			return true;
		} else {
			return false;
		}
	},

	makeTree: (result, el) => {
		const parseJson = parse.filterJson(serializeJson(result.parse));
		const tree = jsonTree.create(parseJson, el);
		tree.expand();
	},
};

const scope = {
	collapseFunctions: true,

	filterJson: obj => {
		if (Array.isArray(obj)) {
			return obj.map(scope.filterJson);
		} else if (typeof obj === 'object') {
			const newObj = {};
			Object.entries(obj).forEach(([key, val]) => {
				newObj[key] = scope.collapseJson(key, scope.filterJson(val));
			});
			return newObj;
		} else {
			return obj;
		}
	},

	collapseJson: (name, obj) => {
		if (scope.collapseFunctions && name.toLowerCase() === 'functions') {
			return Object.keys(obj);
		} else {
			return obj;
		}
	},

	collapseNodes: node => {
		return ![
			'token',
			'tokens',
			'generics',
			'generic',
			'functionbody',
		].includes(node.label.toString().toLowerCase());
	},

	makeTree: (result, el) => {
		// remove global scope
		const scopeJson = scope.filterJson(
			serializeJson(result.scope?.slice(0, result.scope.length))
		);
		const tree = jsonTree.create(scopeJson, el);
		tree.expand(scope.collapseNodes);
	},
};

// converts JS Map type into objects, and stringifies object keys that are nested objects
const serializeJson = json => {
	if (Array.isArray(json)) {
		return json.map(serializeJson);
	} else if (json instanceof Map) {
		const newJson = {};
		json.forEach((val, key) => {
			let newKey = serializeJson(key);
			if (typeof newKey === 'object') {
				if ('String' in newKey) {
					newKey = newKey['String'];
				} else {
					newKey = JSON.stringify(newKey, null, 2).replaceAll(
						'"',
						"'"
					);
				}
			}
			newJson[newKey] = serializeJson(val);
		});
		return newJson;
	} else if (typeof json === 'object') {
		for (const key of Object.keys(json)) {
			json[key] = serializeJson(json[key]);
		}
		return json;
	} else {
		return json;
	}
};

export { semantics, parse, scope, serializeJson };
