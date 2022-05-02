const code = [
	`log ('Testing Cupid Playground v0.1')

type person = [
	string name,
	int age
]

person jane = [
	name: 'Jane Doe',
	age: 34
]

log (jane.name)

# Try uncommenting this:
# int my_number = 'oh no!'
`,

	`trait [t] new = [
	fun [t] new
]

type [t] iter = [
	t entries,
	int index
]

use [t] iter {
	fun [t] next = self => {
		self.index ++
		(self.entries).(self.index)
	}
}

# Implement traits for iterator
use [t] new with iter {
	fun [iter] new = t entries => [
		entries: entries,
		index: 0
	]
}

iter [t: int] myiter = iter.new([1, 2, 3])

myiter.next()`,

	`int num = 10`,
];

const exampleButtons = () => [
	document.getElementById('example-1'),
	document.getElementById('example-2'),
	document.getElementById('example-3'),
];

const bindExampleButtons = func => {
	const buttons = exampleButtons();
	for (const i in buttons) {
		buttons[i].addEventListener('click', func.bind(this, code[i]));
	}
};

export { code, bindExampleButtons };
