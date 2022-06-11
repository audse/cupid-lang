#![feature(proc_macro_diagnostic)]

use syn::{
	Item,
	ItemImpl,
	ImplItem,
	ImplItemMethod,
	spanned::*
};

use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;

/// Allows structs to have unique items in the `Trace` trait by creating a new
/// trait behind the scenes. This kind of defeats the purpose of traits, but
/// really it's just allow better semantic separation of code.
/// 
/// ## Example
/// ```
/// use cupid_trace::trace_this;
/// 
/// #[derive(Debug)]
/// struct MyFirstStruct;
/// struct MySecondStruct;
/// 
/// trait Trace {}
/// 
/// #[trace_this]
/// impl Trace for MyFirstStruct {
///    fn trace_something(&self) {
///         println!("Accessing {self:?}")
///    }
/// }
/// 
/// impl Trace for MySecondStruct {
///     fn trace_something_else(&self) {
///         println!("Doing something with {self:?}")
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn trace_this(_: TokenStream, input: TokenStream) -> TokenStream {
	let input = TokenStream2::from(input);
	let item: Item = syn::parse2(input).expect("failed to parse input");

	match &item {
		Item::Impl(ItemImpl { attrs, generics, self_ty, items, .. }) => {
			match &**self_ty {
				syn::Type::Path(self_ident) => {
					let new_trait = syn::Ident::new(
						&format!("Trace{}", self_ident.path.get_ident().unwrap()), 
						self_ident.span()
					);

					let item_signatures = items.iter().map(|item| match item {
						ImplItem::Method(ImplItemMethod { attrs, vis, sig, ..}) => {
							quote::quote! {
								#(#attrs)* #vis #sig;
							}
						},
						_ => {
							err_misuse::<ImplItem>(item);
							panic!();
						}
					}).collect::<Vec<TokenStream2>>();

					let output = quote::quote! {
						impl Trace for #self_ty {}
						pub trait #new_trait: Trace {
							#(#item_signatures)*
						}
						#(#attrs)*
						impl #generics #new_trait for #self_ty {
							#(#items)*
						}
					};
					output.into()
				},
				_ => {
					err_misuse::<syn::Type>(self_ty);
					quote::quote!(#item).into()
				}
			}
		},
		_ => {
			err_misuse::<Item>(&item);
			quote::quote!(#item).into()
		}
	}
}

/// Adds a trace message to the scope when entering and exiting a function call
/// 
/// ## Example ouput
/// ```no_run
/// --> Declaration::analyze_scope
/// ...
/// <-- Declaration::analyze_scope
/// ```
#[proc_macro_attribute]
pub fn trace(_: TokenStream, input: TokenStream) -> TokenStream {
	let input = TokenStream2::from(input);

	let item: ImplItem = syn::parse2(input).expect("failed to parse input");
	
	if option_env!("RUST_DEBUG") != Some("true") {
		return quote::quote!(#item).into();
	}
	
	match &item {
		ImplItem::Method(ImplItemMethod { attrs, vis, sig, block, ..}) => {
			let ident = &sig.ident;
			let output = if let Some((last_stmt, stmts)) = &block.stmts.split_last() {
				quote::quote! {
					#(#attrs)*
					#vis #sig {
						scope.trace(format!("\n--> {}::{}", fmt_type!(Self), stringify!(#ident)));
						#(#stmts)*
						scope.trace(format!("<-- {}::{}\n", fmt_type!(Self), stringify!(#ident)));
						#last_stmt
					}
				}
			} else {
				quote::quote!{ #(item) }
			};
			output.into()
		}
		_ => {
			err_misuse::<ImplItem>(&item);
			quote::quote!(#item).into()
		}
	}
}

fn err_misuse<T: Spanned>(item: &T) {
	item.span()
		.unstable()
		.error("Trace can only be applied to functions")
		.emit()
}