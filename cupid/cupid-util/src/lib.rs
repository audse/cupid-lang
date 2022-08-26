pub mod bidirectional_iterator;
use std::iter::FilterMap;

pub use bidirectional_iterator::*;

pub mod builder;
pub use builder::*;

pub mod error_codes;
pub use error_codes::*;

pub mod fmt;
pub use fmt::*;

pub mod invert_option;
pub use invert_option::*;

pub type Str = std::borrow::Cow<'static, str>;

/// Shorthand `Box::new(...)` as an associated trait. Implemented
/// for any type that can be boxed.
///
/// # Examples
/// ```
/// use cupid_util::Bx;
///
/// let word = "hello";
/// assert_eq!(word.bx(), Box::new(word));
/// ```
pub trait Bx
where
    Self: Sized,
{
    fn bx(self) -> Box<Self> {
        Box::new(self)
    }
}

impl<T: Sized> Bx for T {}

pub trait BxOption<T>
where
    Self: Sized,
{
    fn bx_option(self) -> Option<Box<T>>;
}

impl<T: Sized> BxOption<T> for Option<T> {
    fn bx_option(self) -> Option<Box<T>> {
        self.map(|t| Box::new(t))
    }
}

/// Shorthand `std::rc::Rc::new(...)` as an associated trait. Implemented
/// for any type that can be inside an Rc.
///
/// # Examples
/// ```
/// use cupid_util::WrapRc;
///
/// let word = "hello";
/// assert_eq!(word.rc(), std::rc::Rc::new(word));
/// ```
pub trait WrapRc
where
    Self: Sized,
{
    fn rc(self) -> std::rc::Rc<Self> {
        std::rc::Rc::new(self)
    }
}

impl<T: Sized> WrapRc for T {}

/// Shorthand `std::cell::RefCell::new(...)` as an associated trait. Implemented
/// for any type that can be inside an RefCell.
///
/// # Examples
/// ```
/// use cupid_util::WrapRefCell;
///
/// let word = "hello";
/// assert_eq!(word.ref_cell(), std::cell::RefCell::new(word));
/// ```
pub trait WrapRefCell
where
    Self: Sized,
{
    fn ref_cell(self) -> std::cell::RefCell<Self> {
        std::cell::RefCell::new(self)
    }
}

impl<T: Sized> WrapRefCell for T {}

/// Filters a list of `Option` to include only `Some` values
/// # Examples
/// ```
/// use cupid_util::FilterSome;
///
/// let nums: Vec<i32> = [Some(1), Some(2), None, Some(4)]
///     .into_iter()
///     .filter_some()
///     .collect();
/// assert!(nums == [1, 2, 4])
/// ```
pub trait FilterSome<T>: Iterator<Item = Option<T>>
where
    Self: Sized,
{
    fn filter_some(self) -> FilterMap<Self, fn(Self::Item) -> Option<T>>;
}

impl<I: Iterator<Item = Option<T>>, T> FilterSome<T> for I {
    fn filter_some(self) -> FilterMap<Self, fn(Self::Item) -> Option<T>> {
        let f = |i: Option<T>| -> Option<T> { i };
        self.filter_map(f)
    }
}

/// Allows chaining iterator `extend` method
/// # Examples
/// ```
/// use cupid_util::Plus;
///
/// let mut nums = vec![1, 2, 3].plus([4, 5, 6]).plus([7, 8, 9]);
/// assert!(nums == [1, 2, 3, 4, 5, 6, 7, 8, 9]);
/// ```
pub trait Plus<A> {
    fn plus<T: IntoIterator<Item = A>>(self, iter: T) -> Self;
}

impl<A, I: Extend<A>> Plus<A> for I {
    fn plus<T: IntoIterator<Item = A>>(mut self, iter: T) -> Self {
        self.extend(iter.into_iter());
        self
    }
}
