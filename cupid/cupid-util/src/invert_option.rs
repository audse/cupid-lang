/// Inverts `Option<Result<T, E>>` into `Result<Option<T>, E>`
///
/// # Examples
/// ```
/// use cupid_util::invert;
/// 
/// type IntResult = Result<(), i32>;
/// let x: Option<IntResult> = Some(Err(1));
/// 
/// assert!(invert(x).is_err());
/// ```
pub fn invert<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

/// Functions the same as `cupid_util::invert` but as an associated function
/// rather than a standalone function. Implemented by default for all option types.
/// 
/// # Examples
/// ```
/// use cupid_util::{invert, InvertOption};
/// 
/// type IntResult = Result<i32, i32>;
/// let x: Option<IntResult> = Some(Err(1));
/// 
/// assert!(x.invert().is_err());
/// ```
pub trait InvertOption<T, E> {
	fn invert(self) -> Result<Option<T>, E>;
}

impl<T, E> InvertOption<T, E> for Option<Result<T, E>> {
	fn invert(self) -> Result<Option<T>, E> {
		self.map_or(Ok(None), |v| v.map(Some))
	}
}