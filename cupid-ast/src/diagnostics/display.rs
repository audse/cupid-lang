use crate::*;
use std::fmt::{
	Display,
	Result,
	Formatter
};


mod nodes;
pub use nodes::*;
mod scope;

pub trait AsTable: Tabled where Self: Clone + ToOwned<Owned = Self> + Sized {
	fn as_table(&self) -> Table {
		vec![self.to_owned()]
			.table()
			.with(Style::modern())
			.with(Modify::new(object::Segment::all()).with(Alignment::left()))
	}
	fn as_named_table(&self, name: &str) -> Table {
		vec![self.to_owned()]
			.table()
			.with(Style::modern())
			.with_header(name)
	}
}

pub trait TableStyle {
	fn with_header(self, header: &str) -> Table;
	fn with_bold_header(self, header: &str) -> Table;
	fn with_footer(self, footer: &str) -> Table;
}

impl TableStyle for Table {
	fn with_header(self, header: &str) -> Self {
		self.with(Header(header))
			.with(Highlight::new(
				object::Rows::first(), 
				Style::modern().frame().bottom_left_corner('├').bottom_right_corner('┤')
			))
			.with(Modify::new(object::Rows::first()).with(Alignment::center()))
	}
	fn with_bold_header(self, header: &str) -> Self {
		self.with_header(header)
			.with(Modify::new(
				object::Rows::first()).with(Format::new(|s| s.bold().to_string())
			))
	}
	fn with_footer(self, footer: &str) -> Table {
		self.with(Footer(footer))
			.with(Highlight::new(
				object::Rows::last(), 
				Style::modern().frame().top_left_corner('├').top_right_corner('┤')
			))
			.with(Modify::new(object::Rows::last()).with(Alignment::center()))
	}
}


pub fn fmt_option<T: Display>(opt: &Option<T>) -> String {
	fmt_option!(opt)
}

pub fn fmt_vec<T: Display>(vec: &[T]) -> String {
	fmt_list!(vec)
		.table()
		.with(Disable::Row(0..1))
		.with(Style::ascii()
			.left_off()
			.top_off()
			.right_off()
			.bottom_off()
		)
		.with(Modify::new(object::Columns::first()).with(Alignment::left()))
		.to_string()
}


#[derive(Debug, Clone, Tabled)]
pub struct TableVec<T: Display + ToOwned<Owned = T>>(
	#[tabled(display_with="fmt_vec")]
	pub Vec<T>
);

#[derive(Debug, Clone, Tabled)]
pub struct TablePair<K: Display + ToOwned<Owned = K>, V: Display + ToOwned<Owned = V>>(pub K, pub V);

impl<K: Display + ToOwned<Owned = K>, V: Display + ToOwned<Owned = V>> Display for TablePair<K, V> {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		write!(f, "{}", self.0)?;
		write!(f, "{}", self.1)
	}
}

pub fn fmt_map<K: Clone + Display + ToOwned<Owned = K>, V: Clone + Display + ToOwned<Owned = V>>(map: &HashMap<K, V>) -> String {
	let map = map.iter().map(|(k, v)| TablePair(k.to_owned(), v.to_owned())).collect::<Vec<TablePair<K, V>>>();
	TableVec(map).0.table()
		.with(
			Modify::new(object::Cell(0, 0))
				.with(Format::new(|_| "key".to_string()))
		)
		.with(
			Modify::new(object::Cell(0, 1))
				.with(Format::new(|_| "value".to_string()))
		)
		.with(Style::modern())
		.to_string()
}