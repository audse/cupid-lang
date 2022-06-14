/// Shorthand for `option_val.as_mut().map(...)` implemented for all option types
/// 
/// # Examples
/// ```
/// use cupid_util::MapMut;
/// let mut x: Option<String> = Some("\n hello world! \n".to_string());
/// assert_eq!(x.map_mut(|s| s.trim()), Some("hello world!"));
/// ```
pub trait MapMut<'a, T: 'a, R: 'a> {
	fn map_mut<F: FnOnce(&'a mut T) -> R>(&'a mut self, f: F) -> Option<R>;
}

impl<'a, T: 'a, R: 'a> MapMut<'a, T, R> for Option<T> {
	fn map_mut<F: FnOnce(&'a mut T) -> R>(&'a mut self, f: F) -> Option<R> {
		self.as_mut().map(f)
	}
}