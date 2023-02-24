/**
 * By Max Leiter
 * jsonTree: A dependency-free lightweight vanilla Javascript library to display JSON in an HTML unordered list.
 **/

const serialize = json => {
	switch (typeof json) {
		case 'array':
			return json.map(serialize);
		case 'object':
			let newObj = {};
			for (const [key, val] of Object.entries(json)) {
				const newKey = key.toLowerCase().replace(/_/g, ' ');
				newObj[newKey] = serialize(val);
			}
			return newObj;
		default:
			if (json instanceof Map) {
				let newObj = {};
				json.forEach((val, key) => {
					const newKey = key.toLowerCase().replace(/_/g, ' ');
					newObj[newKey] = serialize(val);
				});
				return newObj;
			} else {
				return json;
			}
	}
};

class jsonTree {
	/**
	 * json: URL for json file or a JSON object
	 * selector: the elements selector to apply the tree to
	 * depth: bool to add a "depth-#" class, can increase loading times
	 **/
	constructor(json, selector, depth) {
		if (Array.isArray(json)) json = { nodes: json };
		this.generateTree(selector, serialize(json));
		this.classify(selector, depth);
	}

	/** Generate the DOM elements for the tree**/
	generateTree(selector, json) {
		const element = document.querySelector(selector);
		element.classList.add('jsonTree');
		element.innerHTML = this.json2html(json);
		const top = document.querySelector('[data-id="top"]');
		top.addEventListener('click', e => {
			e.preventDefault();
			if (e.target) {
				let targetNode = null;
				switch (e.target.nodeName.toLowerCase()) {
					case 'li':
						targetNode = e.target;
						break;
					case 'span':
						targetNode = e.target.parentNode;
						break;
				}
				if (
					targetNode &&
					Array.from(targetNode.childNodes).length > 1
				) {
					this.toggleClass(targetNode, 'selected');
				}
			}
		});
	}

	classify(selector, depth) {
		this.applyClasses(selector, 'li', 'ul', depth);
		this.applyClasses(selector, 'ul', 'li', depth);
	}

	/** Applies classes to the element, including "parent" and "depth-#" **/
	applyClasses(selector, parent, child, depth) {
		const parents = Array.from(
			document.querySelectorAll(`${selector} ${parent}`)
		);
		parents.forEach(function (element) {
			const filter = Array.from(element.children).filter(
				el =>
					el.tagName.toLowerCase() === child.toLowerCase().toString()
			);
			if (filter.length > 0) {
				// It's a parent!
				element.classList.add('parent');
				element.style.cursor = 'pointer';
			} else {
				element.style.cursor = 'auto';
			}

			// The amount of parents, "#top" is assigned by json2html
			if (depth) {
				const count = depth(element);
				element.classList.add('depth-' + count);
			}
		});
	}

	/** Returns the amount of parents of an element **/
	depth(ele) {
		if (
			ele.parentNode &&
			ele.parentNode.getAttribute('data-id') === 'top'
		) {
			return ele == null ? 0 : 1 + depth(ele.parentNode);
		} else {
			return 0;
		}
	}

	/** Returns a JSON file in HTML tokens **/
	json2html(json) {
		let i,
			html = '';
		json = this.htmlEscape(JSON.stringify(json));
		json = JSON.parse(json);
		html += '<ul data-id="top">';
		for (i in json) {
			html += `<li class="selected"><span class="key">${i}</span> : `;
			if (typeof json[i] === 'object') {
				html += `{ ${this.json2html(json[i])} }`;
			} else html += json[i];
			html += '</li>';
		}
		html += '</ul>';
		return html;
	}

	/** To stop XSS attacks by using JSON with HTML nodes **/
	htmlEscape(str) {
		const tagsToReplace = {
			'&': '&amp;',
			'<': '&lt;',
			'>': '&gt;',
		};
		return str.replace(/[&<>]/g, function (tag) {
			return tagsToReplace[tag] || tag;
		});
	}

	/** Toggles an elements class **/
	toggleClass(el, className) {
		if (el) {
			el.classList.toggle(className);
		}
	}
}

export default jsonTree;
