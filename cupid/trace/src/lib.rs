#![feature(proc_macro_diagnostic)]

use syn::spanned::*;

use proc_macro2::TokenStream as TokenStream2;
use proc_macro::TokenStream;

/// Prints a trace message when entering and exiting a function call
/// 
/// # Example output
/// ```no_run
/// --> Declaration::analyze_scope
/// ...
/// <-- Declaration::analyze_scope
/// ```
#[proc_macro_attribute]
pub fn trace(_: TokenStream, input: TokenStream) -> TokenStream {
	let input = TokenStream2::from(input);

	let item: syn::ImplItem = syn::parse2(input).expect("failed to parse input");
	
	if option_env!("CUPID_DEBUG") != Some("true") {
		return quote::quote!(#item).into();
	}
	
	match &item {
		syn::ImplItem::Method(syn::ImplItemMethod { attrs, vis, sig, block, ..}) => {
			let ident = &sig.ident;
			let output = if let Some((last_stmt, stmts)) = &block.stmts.split_last() {
				quote::quote! {
					#(#attrs)*
					#vis #sig {
						println!("\n--> {}::{}", cupid_util::fmt_type!(Self), stringify!(#ident));
						#(#stmts)*
						let _trace_result = {
							#last_stmt
						};
						println!(
							"<-- {}::{}\n", 
							cupid_util::fmt_type!(Self), 
							stringify!(#ident), 
						);
						_trace_result
					}
				}
			} else {
				quote::quote!{ #(item) }
			};
			output.into()
		}
		_ => {
			err_misuse::<syn::ImplItem>(&item);
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